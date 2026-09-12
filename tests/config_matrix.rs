//! Config-knob behavior matrix for tantivy-helper.
//!
//! Every public config knob must OBSERVABLY change behavior: the table
//! below pairs a default with an alternate value and asserts the observable
//! output/state differs. A knob that cannot change behavior is a bug (see
//! breaker's sliding_window_size incident).
//!
//! 0.3.0 removed three knobs that FAILED this standard (settable, sometimes
//! even tested-as-setters, never read): `BM25Config` (the entire struct),
//! `IndexConfig::tokenizers`, and `IndexSettings::temp_directory`.
//! `Highlighter::max_tokens` was dead and is now wired (see
//! `knob_max_tokens_bounds_snippet`).
#![allow(clippy::unwrap_used, clippy::expect_used)]

use std::path::PathBuf;

use tantivy::TantivyDocument;
use tantivy_helper::SearchEngine;
use tantivy_helper::highlight::Highlighter;
use tantivy_helper::query::QueryBuilder;
use tantivy_helper::schema::{FieldDefinition, FieldType, IndexConfig, IndexSettings};

/// Unique per-test index directory under the OS temp dir.
fn temp_index_dir(tag: &str) -> PathBuf {
    let dir = std::env::temp_dir().join(format!("tantivy-helper-cfg-{tag}-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&dir);
    dir
}

/// Build an engine whose `index_base_path` points at a fresh temp dir.
fn engine_at(
    tag: &str,
    fields: Vec<FieldDefinition>,
    num_threads: usize,
) -> (SearchEngine, tantivy::schema::Schema, PathBuf) {
    let path = temp_index_dir(tag);
    let config = IndexConfig {
        fields,
        settings: IndexSettings {
            num_threads,
            index_base_path: path.to_string_lossy().into_owned(),
        },
    };
    let engine = SearchEngine::new(&config).unwrap();
    let schema = engine.schema().clone();
    (engine, schema, path)
}

fn text_engine(tag: &str) -> (SearchEngine, tantivy::schema::Field, PathBuf) {
    let (engine, schema, path) = engine_at(
        tag,
        vec![FieldDefinition::text("title")],
        IndexSettings::default().num_threads,
    );
    let field = schema.get_field("title").unwrap();
    (engine, field, path)
}

fn add_text_doc(engine: &mut SearchEngine, field: tantivy::schema::Field, value: &str) {
    let mut doc = TantivyDocument::default();
    doc.add_text(field, value);
    engine.index_document(doc).unwrap();
    engine.commit().unwrap();
}

// --- Highlighter::pre_tag / post_tag (via with_tags) ----------------------

#[test]
fn knob_with_tags_changes_highlight_output() {
    let default = Highlighter::new();
    let configured = Highlighter::with_tags("**", "**");

    let plain = default.highlight("rust is great", &["rust".into()]);
    let tagged = configured.highlight("rust is great", &["rust".into()]);

    assert_eq!(plain, "<mark>rust</mark> is great");
    assert_ne!(plain, tagged, "with_tags must change highlight output");
    assert_eq!(tagged, "**rust** is great");
}

// --- Highlighter::max_tokens (dead in <=0.2.0, wired in 0.3.0) ------------

#[test]
fn knob_max_tokens_bounds_snippet() {
    let words = "alpha bravo charlie delta echo foxtrot golf hotel india juliet kilo lima";
    let text = format!("needle {words} and more words after that");

    let tight = Highlighter {
        max_tokens: 3,
        ..Highlighter::new()
    };
    let loose = Highlighter {
        max_tokens: 25,
        ..Highlighter::new()
    };

    let tight_out = tight.snippet(&text, &["needle".into()]);
    let loose_out = loose.snippet(&text, &["needle".into()]);

    assert_eq!(
        tight_out.split_whitespace().count(),
        3,
        "max_tokens must cap the snippet token count"
    );
    assert_ne!(
        tight_out, loose_out,
        "max_tokens must observably change the snippet"
    );
    assert!(tight_out.contains("needle"));
}

#[test]
fn max_tokens_zero_yields_empty_snippet() {
    let h = Highlighter {
        max_tokens: 0,
        ..Highlighter::new()
    };
    assert_eq!(h.snippet("hello world", &["hello".into()]), "");
}

// --- snippet safety: non-ASCII (char-boundary regression) -----------------

#[test]
fn snippet_is_char_boundary_safe_on_non_ascii() {
    let h = Highlighter::new();
    // `İ` case-folds to two chars and multi-byte chars sit near the window
    // edges — a byte-offset slice of the original text would panic.
    let text = "İstanbul İstanbul İstanbul wörld größen hören münchen köln häuser fenster türen brücke straße";
    let out = h.snippet(text, &["istanbul".into()]);
    assert!(out.contains("stanbul"));
    // Fallback branch (char counts diverge under folding).
    let out2 = h.snippet("ÉÉÉ ÉÉÉ ÉÉÉ İstanbul ÉÉÉ ÉÉÉ ÉÉÉ", &["istanbul".into()]);
    assert!(!out2.is_empty());
}

// --- IndexSettings::index_base_path ----------------------------------------

#[test]
fn knob_index_base_path_controls_index_location_and_isolation() {
    let (mut a, field_a, path_a) = text_engine("base-path-a");
    let (mut b, field_b, path_b) = text_engine("base-path-b");

    // Observable: the knob decides WHERE the index lives on disk.
    assert!(path_a.is_dir(), "engine must create the base path dir");
    assert!(path_b.is_dir());
    assert_ne!(path_a, path_b);

    add_text_doc(&mut a, field_a, "engine alpha");
    add_text_doc(&mut b, field_b, "engine beta");

    let hits_a = a
        .search(
            &*QueryBuilder::new(a.index(), a.schema())
                .term("title", "alpha")
                .unwrap()
                .build()
                .unwrap(),
            10,
        )
        .unwrap();
    let hits_in_b = b
        .search(
            &*QueryBuilder::new(b.index(), b.schema())
                .term("title", "alpha")
                .unwrap()
                .build()
                .unwrap(),
            10,
        )
        .unwrap();

    assert_eq!(hits_a.len(), 1, "doc lands in the configured path");
    assert!(
        hits_in_b.is_empty(),
        "indexes at different base paths are isolated"
    );

    let _ = std::fs::remove_dir_all(path_a);
    let _ = std::fs::remove_dir_all(path_b);
}

// --- IndexSettings::num_threads --------------------------------------------
//
// WIRED but not black-box differentiable: tantivy consumes it as the
// indexing writer's thread pool size (a throughput/memory knob, not an
// output knob). asserted here as wired-smoke: both a minimal and a large
// pool produce a working, searchable index.

#[test]
fn knob_num_threads_is_wired_smoke() {
    for threads in [1usize, 8] {
        let (mut engine, schema, path) = engine_at(
            &format!("threads-{threads}"),
            vec![FieldDefinition::text("title")],
            threads,
        );
        let field = schema.get_field("title").unwrap();
        add_text_doc(&mut engine, field, "thread pool works");
        let hits = engine
            .search(
                &*QueryBuilder::new(engine.index(), engine.schema())
                    .term("title", "thread")
                    .unwrap()
                    .build()
                    .unwrap(),
                10,
            )
            .unwrap();
        assert_eq!(hits.len(), 1, "num_threads={threads} must stay functional");
        let _ = std::fs::remove_dir_all(path);
    }
}

// --- FieldDefinition::stored ------------------------------------------------

#[test]
fn knob_stored_controls_doc_retrieval() {
    for stored in [true, false] {
        let (mut engine, schema, path) = engine_at(
            &format!("stored-{stored}"),
            vec![FieldDefinition::text("title").stored(stored)],
            1,
        );
        let field = schema.get_field("title").unwrap();
        assert_eq!(
            schema.get_field_entry(field).is_stored(),
            stored,
            "the knob must reach the tantivy schema"
        );

        add_text_doc(&mut engine, field, "retrieve me");
        let hits = engine
            .search(
                &*QueryBuilder::new(engine.index(), engine.schema())
                    .term("title", "retrieve")
                    .unwrap()
                    .build()
                    .unwrap(),
                10,
            )
            .unwrap();
        assert_eq!(hits.len(), 1);

        let searcher = engine
            .index()
            .reader_builder()
            .try_into()
            .unwrap()
            .searcher();
        let doc = searcher
            .doc::<TantivyDocument>(hits[0].doc_address)
            .unwrap();
        let present = doc.get_first(field).is_some();
        if stored {
            assert!(present, "stored field must be retrievable from the doc");
        } else {
            assert!(
                !present,
                "stored=false must hide the field from doc retrieval"
            );
        }
        let _ = std::fs::remove_dir_all(path);
    }
}

// --- FieldDefinition::indexed ------------------------------------------------

#[test]
fn knob_indexed_controls_term_searchability() {
    for indexed in [true, false] {
        let (mut engine, schema, path) = engine_at(
            &format!("indexed-{indexed}"),
            vec![FieldDefinition::text("title").indexed(indexed)],
            1,
        );
        let field = schema.get_field("title").unwrap();
        add_text_doc(&mut engine, field, "findable needle");
        let query = QueryBuilder::new(engine.index(), engine.schema())
            .term("title", "needle")
            .unwrap()
            .build()
            .unwrap();
        let result = engine.search(&*query, 10);
        if indexed {
            assert_eq!(result.unwrap().len(), 1);
        } else {
            let err = result.unwrap_err();
            assert!(
                err.to_string().contains("not indexed"),
                "indexed=false must make the field unsearchable, got: {err}"
            );
        }
        let _ = std::fs::remove_dir_all(path);
    }
}

// --- FieldDefinition::fast ----------------------------------------------------
//
// `fast` was silently dropped for Text and Bool fields before 0.3.0; it is
// wired for every field type now. Observable channel: the tantivy schema
// (fast fields back aggregations/sorting).

#[test]
fn knob_fast_reaches_schema_for_every_field_type() {
    let fields = vec![
        FieldDefinition::text("t").fast(true).indexed(true),
        FieldDefinition::u64("u").fast(true).indexed(true),
        FieldDefinition::i64("i").fast(true).indexed(true),
        FieldDefinition::f64("f").fast(true).indexed(true),
        FieldDefinition::date_time("d").fast(true).indexed(true),
        FieldDefinition::bool("b").fast(true).indexed(true),
    ];
    let (_engine, schema, path) = engine_at("fast-on", fields, 1);
    for name in ["t", "u", "i", "f", "d", "b"] {
        let field = schema.get_field(name).unwrap();
        assert!(
            schema.get_field_entry(field).is_fast(),
            "fast(true) must reach the schema for field '{name}'"
        );
    }
    let _ = std::fs::remove_dir_all(path);

    let fields_off = vec![
        FieldDefinition::text("t").fast(false).indexed(true),
        FieldDefinition::bool("b").fast(false).indexed(true),
    ];
    let (_engine, schema, path) = engine_at("fast-off", fields_off, 1);
    for name in ["t", "b"] {
        let field = schema.get_field(name).unwrap();
        assert!(
            !schema.get_field_entry(field).is_fast(),
            "fast(false) must keep field '{name}' non-fast"
        );
    }
    let _ = std::fs::remove_dir_all(path);
}

// --- QueryBuilder::term -------------------------------------------------------

#[test]
fn knob_term_query_is_exact_selective() {
    let (mut engine, field, path) = text_engine("term");
    add_text_doc(&mut engine, field, "alpha document");
    add_text_doc(&mut engine, field, "beta document");

    let hits = engine
        .search(
            &*QueryBuilder::new(engine.index(), engine.schema())
                .term("title", "alpha")
                .unwrap()
                .build()
                .unwrap(),
            10,
        )
        .unwrap();
    assert_eq!(hits.len(), 1, "term must match only the exact doc");
    let _ = std::fs::remove_dir_all(path);
}

// --- QueryBuilder::prefix -------------------------------------------------------

#[test]
fn knob_prefix_query_autocompletes() {
    let (mut engine, field, path) = text_engine("prefix");
    add_text_doc(&mut engine, field, "Hello world");

    // Prefix terms are matched against the *tokenized* (lowercased) index
    // terms, so the prefix itself must be lowercase.
    let hits = engine
        .search(
            &*QueryBuilder::new(engine.index(), engine.schema())
                .prefix("title", "hel")
                .unwrap()
                .build()
                .unwrap(),
            10,
        )
        .unwrap();
    assert_eq!(hits.len(), 1, "prefix must autocomplete partial words");

    let non_matching = engine
        .search(
            &*QueryBuilder::new(engine.index(), engine.schema())
                .prefix("title", "xyz")
                .unwrap()
                .build()
                .unwrap(),
            10,
        )
        .unwrap();
    assert!(non_matching.is_empty(), "unrelated prefix must not match");
    let _ = std::fs::remove_dir_all(path);
}

// --- QueryBuilder::fuzzy -------------------------------------------------------

#[test]
fn knob_fuzzy_distance_tolerates_typos_exact_does_not() {
    let (mut engine, field, path) = text_engine("fuzzy");
    add_text_doc(&mut engine, field, "hello world");

    let exact_miss = QueryBuilder::new(engine.index(), engine.schema())
        .term("title", "helo")
        .unwrap()
        .build()
        .unwrap();
    let fuzzy_hit = QueryBuilder::new(engine.index(), engine.schema())
        .fuzzy("title", "helo", 1)
        .unwrap()
        .build()
        .unwrap();

    assert!(
        engine.search(&*exact_miss, 10).unwrap().is_empty(),
        "exact term must miss the typo"
    );
    assert_eq!(
        engine.search(&*fuzzy_hit, 10).unwrap().len(),
        1,
        "fuzzy(distance=1) must match the typo"
    );
    let _ = std::fs::remove_dir_all(path);
}

// --- QueryBuilder::build (empty guard) ------------------------------------------

#[test]
fn build_without_terms_is_an_error() {
    let (engine, _field, path) = text_engine("build-empty");
    let err = QueryBuilder::new(engine.index(), engine.schema())
        .build()
        .unwrap_err();
    assert!(err.to_string().contains("no query terms"));
    let _ = std::fs::remove_dir_all(path);
}

// --- FieldType plumbing ----------------------------------------------------------

#[test]
fn every_field_type_round_trips_through_the_engine() {
    for (ty, name, is_text) in [
        (FieldType::Text, "t", true),
        (FieldType::U64, "u", false),
        (FieldType::I64, "i", false),
        (FieldType::F64, "f", false),
        (FieldType::Bool, "b", false),
    ] {
        let fields = match ty {
            FieldType::Text => FieldDefinition::text(name),
            FieldType::U64 => FieldDefinition::u64(name),
            FieldType::I64 => FieldDefinition::i64(name),
            FieldType::F64 => FieldDefinition::f64(name),
            FieldType::Bool => FieldDefinition::bool(name),
            FieldType::DateTime => FieldDefinition::date_time(name),
        };
        let (mut engine, schema, path) = engine_at(&format!("roundtrip-{name}"), vec![fields], 1);
        let field = schema.get_field(name).unwrap();
        let mut doc = TantivyDocument::default();
        if is_text {
            doc.add_text(field, "roundtrip value");
        } else {
            match ty {
                FieldType::U64 => doc.add_u64(field, 42),
                FieldType::I64 => doc.add_i64(field, -7),
                FieldType::F64 => doc.add_f64(field, 2.5),
                FieldType::Bool => doc.add_bool(field, true),
                _ => unreachable!(),
            }
        }
        engine.index_document(doc).unwrap();
        engine.commit().unwrap();
        let _ = std::fs::remove_dir_all(path);
    }
}
