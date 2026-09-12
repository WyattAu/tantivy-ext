# Requirements — tantivy-ext

Numbered, testable requirements. Every requirement maps to at least one named
test; every security-relevant test cites at least one requirement. Threat
IDs reference `THREAT-MODEL.md`.

Scope note: `tantivy-helper` provides full-text search for Rust — a tantivy
0.26 wrapper with typed schemas, autocomplete (prefix) queries, fuzzy
search, highlighting/snippets, and typed query building. Crate name:
`tantivy-helper`; repository: `tantivy-ext`.

Note: v0.3.0 removed `BM25Config` and its knobs (`k1`/`b`/`FieldBoost`/
`recency_boost`) — they were never read by the engine (REQ-TH-003,
REQ-TH-102 retired). Ranking is tantivy's built-in BM25; see CHANGELOG.

## Functional

| ID | Requirement | Priority |
|----|-------------|----------|
| REQ-TH-001 | `SearchEngine` builds over an `IndexConfig` with typed `FieldDefinition`s (`text`, `i64`, `u64`, `f64`, `bool`, `date_time`) supporting `indexed`/`stored`/`fast` modifiers | MUST |
| REQ-TH-002 | `QueryBuilder` constructs typed queries: `term`, `prefix` (autocomplete), `fuzzy` | MUST |
| REQ-TH-003 | *(retired 0.3.0 — dead knob removed)* | — |
| REQ-TH-004 | `Highlighter` wraps matched terms with configurable tags and produces snippets anchored on matches, capped at `max_tokens`; no-match inputs return deterministic output | MUST |
| REQ-TH-005 | `SearchEngine::index_document`/`commit`/`delete` manage the index lifecycle; `search` returns `SearchResult`s | MUST |
| REQ-TH-006 | `SearchError` covers schema, query, index, and IO failures | MUST |

## Security

| ID | Requirement | Priority |
|----|-------------|----------|
| REQ-TH-100 | User query text never reaches tantivy's query *parser*: all queries are built programmatically via the typed builder (T1) | MUST |
| REQ-TH-101 | No panic path on hostile input: empty text/terms, repeated terms, non-ASCII/case-folding input, and custom tags are handled in highlight/snippet/search paths (T2) | MUST |
| REQ-TH-102 | *(retired 0.3.0 — dead knob removed)* | — |
| REQ-TH-103 | Schema field defaults are pinned per type so a field cannot silently lose indexing/storage behavior (T4) | MUST |

## Robustness

| ID | Requirement | Priority |
|----|-------------|----------|
| REQ-TH-200 | Error variants are distinct, `Debug`- and `Display`-formatted for operator diagnostics | SHOULD |
| REQ-TH-201 | `IndexConfig`/`IndexSettings`/`FieldType` are `Debug`-comparable configuration data | SHOULD |

## Traceability Matrix

| Requirement | Test (fn, file) | Property class |
|-------------|-----------------|----------------|
| REQ-TH-001 | `field_def_*_defaults`, `field_def_indexed_only`, `field_def_stored_only`, `field_def_chained_modifiers` (`src/lib.rs`, `tests/integration.rs`); behavior: `knob_stored_controls_doc_retrieval`, `knob_indexed_controls_term_searchability`, `knob_fast_reaches_schema_for_every_field_type`, `knob_index_base_path_controls_index_location_and_isolation`, `knob_num_threads_is_wired_smoke` (`tests/config_matrix.rs`) | unit/integration |
| REQ-TH-002 | `knob_term_query_is_exact_selective`, `knob_prefix_query_autocompletes`, `knob_fuzzy_distance_tolerates_typos_exact_does_not`, `build_without_terms_is_an_error` (`tests/config_matrix.rs`) | integration |
| REQ-TH-003 | *(retired 0.3.0)* | — |
| REQ-TH-004 | `highlighter_*`, `highlight_*`, `snippet_*` (`src/lib.rs`, `tests/integration.rs`); `knob_with_tags_changes_highlight_output`, `knob_max_tokens_bounds_snippet`, `max_tokens_zero_yields_empty_snippet`, `snippet_is_char_boundary_safe_on_non_ascii` (`tests/config_matrix.rs`) | unit/integration |
| REQ-TH-005 | `index_config_with_fields`, `index_settings_defaults`, `every_field_type_round_trips_through_the_engine` (`tests/config_matrix.rs`) | unit/integration |
| REQ-TH-006 | `search_error_variants_distinct`, `search_error_schema_display`, `search_error_query_display`, `search_error_index_display`, `search_error_io_display`, `search_error_debug_format` | unit |
| REQ-TH-100 | Typed-builder query construction suite (`src/query.rs`); `knob_*_query_*` (`tests/config_matrix.rs`) | design/integration |
| REQ-TH-101 | `highlight_empty_text`, `highlight_empty_terms`, `highlight_no_match`, `highlight_repeated_term`, `snippet_no_match_returns_start`, `snippet_is_char_boundary_safe_on_non_ascii` | unit/integration |
| REQ-TH-102 | *(retired 0.3.0)* | — |
| REQ-TH-103 | `field_def_*_defaults` suite | unit |
| REQ-TH-200 | `search_error_variants_distinct`, `search_error_debug_format`, `search_error_*_display` | unit |
| REQ-TH-201 | `field_type_debug_format`, `field_type_equality_and_inequality`, `index_config_debug_format` (`tests/proptest.rs`) | unit/property |

## Test Count

- 75+ `#[test]` functions across unit, integration, and property suites
  (config matrix added, dead BM25 setter tests removed).
- All-features suite passes with 0 failures; no-default-features suite passes.
