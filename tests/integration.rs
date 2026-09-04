//! Integration tests for the tantivy-ext (tantivy-helper) crate.
//!
//! Tests FieldDefinition creation for each type, BM25Config defaults,
//! Highlighter highlight and snippet, and SearchError display.

use tantivy_helper::SearchError;
use tantivy_helper::highlight::Highlighter;
use tantivy_helper::ranking::BM25Config;
use tantivy_helper::schema::{FieldDefinition, FieldType, IndexConfig, IndexSettings};

// ---------------------------------------------------------------------------
// FieldDefinition creation for each type
// ---------------------------------------------------------------------------

#[test]
fn field_def_text_defaults() {
    let f = FieldDefinition::text("title");
    assert_eq!(f.name, "title");
    assert_eq!(f.field_type, FieldType::Text);
    assert!(f.stored);
    assert!(f.indexed);
    assert!(!f.fast);
}

#[test]
fn field_def_u64_defaults() {
    let f = FieldDefinition::u64("views");
    assert_eq!(f.name, "views");
    assert_eq!(f.field_type, FieldType::U64);
    assert!(f.stored);
    assert!(!f.indexed);
    assert!(f.fast);
}

#[test]
fn field_def_i64_defaults() {
    let f = FieldDefinition::i64("timestamp");
    assert_eq!(f.name, "timestamp");
    assert_eq!(f.field_type, FieldType::I64);
    assert!(f.stored);
    assert!(!f.indexed);
    assert!(f.fast);
}

#[test]
fn field_def_f64_defaults() {
    let f = FieldDefinition::f64("score");
    assert_eq!(f.name, "score");
    assert_eq!(f.field_type, FieldType::F64);
    assert!(f.stored);
    assert!(!f.indexed);
    assert!(f.fast);
}

#[test]
fn field_def_date_time_defaults() {
    let f = FieldDefinition::date_time("created_at");
    assert_eq!(f.name, "created_at");
    assert_eq!(f.field_type, FieldType::DateTime);
    assert!(f.stored);
    assert!(!f.indexed);
    assert!(f.fast);
}

#[test]
fn field_def_bool_defaults() {
    let f = FieldDefinition::bool("is_active");
    assert_eq!(f.name, "is_active");
    assert_eq!(f.field_type, FieldType::Bool);
    assert!(f.stored);
    assert!(!f.indexed);
    assert!(f.fast);
}

#[test]
fn field_def_chained_modifiers() {
    let f = FieldDefinition::text("body")
        .stored(false)
        .indexed(false)
        .fast(true);
    assert!(!f.stored);
    assert!(!f.indexed);
    assert!(f.fast);
}

#[test]
fn field_def_stored_only() {
    let f = FieldDefinition::u64("id").stored(true).fast(false);
    assert!(f.stored);
    assert!(!f.fast);
}

#[test]
fn field_def_indexed_only() {
    let f = FieldDefinition::i64("count").indexed(true).fast(false);
    assert!(f.indexed);
    assert!(!f.fast);
}

// ---------------------------------------------------------------------------
// FieldType enum
// ---------------------------------------------------------------------------

#[test]
fn field_type_equality_and_inequality() {
    assert_eq!(FieldType::Text, FieldType::Text);
    assert_eq!(FieldType::U64, FieldType::U64);
    assert_eq!(FieldType::I64, FieldType::I64);
    assert_eq!(FieldType::F64, FieldType::F64);
    assert_eq!(FieldType::DateTime, FieldType::DateTime);
    assert_eq!(FieldType::Bool, FieldType::Bool);

    assert_ne!(FieldType::Text, FieldType::U64);
    assert_ne!(FieldType::I64, FieldType::F64);
    assert_ne!(FieldType::DateTime, FieldType::Bool);
}

#[test]
fn field_type_debug_format() {
    assert_eq!(format!("{:?}", FieldType::Text), "Text");
    assert_eq!(format!("{:?}", FieldType::U64), "U64");
    assert_eq!(format!("{:?}", FieldType::I64), "I64");
    assert_eq!(format!("{:?}", FieldType::F64), "F64");
    assert_eq!(format!("{:?}", FieldType::DateTime), "DateTime");
    assert_eq!(format!("{:?}", FieldType::Bool), "Bool");
}

// ---------------------------------------------------------------------------
// BM25Config defaults and builder
// ---------------------------------------------------------------------------

#[test]
fn bm25_default_values() {
    let cfg = BM25Config::default();
    assert_eq!(cfg.k1, 1.2);
    assert_eq!(cfg.b, 0.75);
    assert!(cfg.field_boosts.is_empty());
    assert_eq!(cfg.recency_boost, 0.0);
}

#[test]
fn bm25_new_matches_default() {
    let cfg = BM25Config::new();
    assert_eq!(cfg.k1, 1.2);
    assert_eq!(cfg.b, 0.75);
}

#[test]
fn bm25_builder_k1() {
    let cfg = BM25Config::new().k1(2.5);
    assert_eq!(cfg.k1, 2.5);
    assert_eq!(cfg.b, 0.75); // unchanged
}

#[test]
fn bm25_builder_b() {
    let cfg = BM25Config::new().b(0.3);
    assert_eq!(cfg.b, 0.3);
    assert_eq!(cfg.k1, 1.2); // unchanged
}

#[test]
fn bm25_builder_field_boosts() {
    let cfg = BM25Config::new()
        .field_boost("title", 2.0)
        .field_boost("body", 0.5);

    assert_eq!(cfg.field_boosts.len(), 2);
    assert_eq!(cfg.field_boosts[0].field, "title");
    assert_eq!(cfg.field_boosts[0].weight, 2.0);
    assert_eq!(cfg.field_boosts[1].field, "body");
    assert_eq!(cfg.field_boosts[1].weight, 0.5);
}

#[test]
fn bm25_builder_recency_boost() {
    let cfg = BM25Config::new().recency_boost(0.2);
    assert_eq!(cfg.recency_boost, 0.2);
}

#[test]
fn bm25_builder_full_chain() {
    let cfg = BM25Config::new()
        .k1(3.0)
        .b(0.6)
        .field_boost("title", 1.5)
        .field_boost("content", 0.8)
        .recency_boost(0.1);

    assert_eq!(cfg.k1, 3.0);
    assert_eq!(cfg.b, 0.6);
    assert_eq!(cfg.field_boosts.len(), 2);
    assert_eq!(cfg.recency_boost, 0.1);
}

// ---------------------------------------------------------------------------
// Highlighter highlight and snippet
// ---------------------------------------------------------------------------

#[test]
fn highlighter_default_tags() {
    let h = Highlighter::new();
    assert_eq!(h.pre_tag, "<mark>");
    assert_eq!(h.post_tag, "</mark>");
    assert_eq!(h.max_tokens, 20);
}

#[test]
fn highlighter_default_trait() {
    let h = Highlighter::default();
    assert_eq!(h.pre_tag, "<mark>");
    assert_eq!(h.post_tag, "</mark>");
}

#[test]
fn highlighter_custom_tags() {
    let h = Highlighter::with_tags("**", "**");
    assert_eq!(h.pre_tag, "**");
    assert_eq!(h.post_tag, "**");
}

#[test]
fn highlight_single_term() {
    let h = Highlighter::new();
    let result = h.highlight("the quick brown fox", &["quick".into()]);
    assert_eq!(result, "the <mark>quick</mark> brown fox");
}

#[test]
fn highlight_multiple_terms() {
    let h = Highlighter::new();
    let result = h.highlight("the quick brown fox", &["quick".into(), "fox".into()]);
    assert_eq!(result, "the <mark>quick</mark> brown <mark>fox</mark>");
}

#[test]
fn highlight_no_match() {
    let h = Highlighter::new();
    let result = h.highlight("the quick brown fox", &["elephant".into()]);
    assert_eq!(result, "the quick brown fox");
}

#[test]
fn highlight_custom_tags_star() {
    let h = Highlighter::with_tags("**", "**");
    let result = h.highlight("rust is great", &["rust".into()]);
    assert_eq!(result, "**rust** is great");
}

#[test]
fn highlight_empty_text() {
    let h = Highlighter::new();
    let result = h.highlight("", &["term".into()]);
    assert_eq!(result, "");
}

#[test]
fn highlight_empty_terms() {
    let h = Highlighter::new();
    let result = h.highlight("hello world", &[]);
    assert_eq!(result, "hello world");
}

#[test]
fn highlight_repeated_term() {
    let h = Highlighter::new();
    let result = h.highlight("go go go", &["go".into()]);
    assert_eq!(result, "<mark>go</mark> <mark>go</mark> <mark>go</mark>");
}

#[test]
fn snippet_contains_match_near_position() {
    let h = Highlighter::new();
    let text = "Start of document. This is the important section about search algorithms. End of document.";
    let result = h.snippet(text, &["search".into()]);
    assert!(result.contains("<mark>search</mark>"));
}

#[test]
fn snippet_no_match_returns_start() {
    let h = Highlighter::new();
    let text = "Alpha Beta Gamma Delta";
    let result = h.snippet(text, &["zzz".into()]);
    // No match, pos defaults to 0
    assert!(result.contains("Alpha") || result.contains("alpha"));
}

#[test]
fn snippet_with_custom_tags() {
    let h = Highlighter::with_tags("**", "**");
    let text = "The search engine handles queries efficiently";
    let result = h.snippet(text, &["search".into()]);
    assert!(result.contains("**search**"));
}

// ---------------------------------------------------------------------------
// SearchError display
// ---------------------------------------------------------------------------

#[test]
fn search_error_index_display() {
    let err = SearchError::Index("commit failed".into());
    assert_eq!(err.to_string(), "index error: commit failed");
}

#[test]
fn search_error_query_display() {
    let err = SearchError::Query("invalid syntax".into());
    assert_eq!(err.to_string(), "query error: invalid syntax");
}

#[test]
fn search_error_schema_display() {
    let err = SearchError::Schema("duplicate field".into());
    assert_eq!(err.to_string(), "schema error: duplicate field");
}

#[test]
fn search_error_io_display() {
    let io_err = std::io::Error::new(std::io::ErrorKind::PermissionDenied, "access denied");
    let err = SearchError::Io(io_err);
    assert_eq!(err.to_string(), "io error: access denied");
}

#[test]
fn search_error_debug_format() {
    let err = SearchError::Index("test".into());
    let debug = format!("{:?}", err);
    assert!(debug.contains("Index"));
}

#[test]
fn search_error_variants_distinct() {
    let e1 = SearchError::Index("a".into());
    let e2 = SearchError::Query("b".into());
    let e3 = SearchError::Schema("c".into());

    assert!(e1.to_string() != e2.to_string());
    assert!(e2.to_string() != e3.to_string());
    assert!(e1.to_string() != e3.to_string());
}

// ---------------------------------------------------------------------------
// IndexSettings and IndexConfig
// ---------------------------------------------------------------------------

#[test]
fn index_settings_defaults() {
    let s = IndexSettings::default();
    assert_eq!(s.num_threads, 4);
    assert!(s.temp_directory.is_none());
    assert_eq!(s.index_base_path, "./tantivy-index");
}

#[test]
fn index_config_with_fields() {
    let config = IndexConfig {
        fields: vec![
            FieldDefinition::text("title"),
            FieldDefinition::u64("id"),
            FieldDefinition::bool("active"),
        ],
        tokenizers: vec![],
        settings: IndexSettings::default(),
    };

    assert_eq!(config.fields.len(), 3);
    assert_eq!(config.fields[0].name, "title");
    assert_eq!(config.fields[1].name, "id");
    assert_eq!(config.fields[2].name, "active");
}

#[test]
fn index_config_debug_format() {
    let config = IndexConfig {
        fields: vec![FieldDefinition::text("body")],
        tokenizers: vec!["custom".into()],
        settings: IndexSettings {
            num_threads: 2,
            temp_directory: Some("/tmp".into()),
            index_base_path: "/data/index".into(),
        },
    };

    let debug = format!("{:?}", config);
    assert!(debug.contains("body"));
    assert!(debug.contains("custom"));
    assert!(debug.contains("/data/index"));
}
