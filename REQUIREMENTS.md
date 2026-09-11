# Requirements — tantivy-ext

Numbered, testable requirements. Every requirement maps to at least one named
test; every security-relevant test cites at least one requirement. Threat
IDs reference `THREAT-MODEL.md`.

Scope note: `tantivy-helper` provides full-text search for Rust — a tantivy
0.26 wrapper with typed schemas, BM25 ranking controls, autocomplete
(prefix) queries, fuzzy search, highlighting/snippets, and typed query
building. Crate name: `tantivy-helper`; repository: `tantivy-ext`.

## Functional

| ID | Requirement | Priority |
|----|-------------|----------|
| REQ-TH-001 | `SearchEngine` builds over an `IndexConfig` with typed `FieldDefinition`s (`text`, `i64`, `u64`, `f64`, `bool`, `date_time`) supporting `indexed`/`stored`/`fast` modifiers | MUST |
| REQ-TH-002 | `QueryBuilder` constructs typed queries: `term`, `prefix` (autocomplete), `fuzzy`, with per-field boosts | MUST |
| REQ-TH-003 | `BM25Config` controls ranking: `b`, `k1`, per-field `FieldBoost`, and `recency_boost`; defaults are sane and constructible | MUST |
| REQ-TH-004 | `Highlighter` wraps matched terms with configurable tags and produces snippets anchored on matches; no-match inputs return deterministic output | MUST |
| REQ-TH-005 | `SearchEngine::index_document`/`commit`/`delete` manage the index lifecycle; `search` returns `SearchResult`s | MUST |
| REQ-TH-006 | `SearchError` covers schema, query, index, and IO failures | MUST |

## Security

| ID | Requirement | Priority |
|----|-------------|----------|
| REQ-TH-100 | User query text never reaches tantivy's query *parser*: all queries are built programmatically via the typed builder (T1) | MUST |
| REQ-TH-101 | No panic path on hostile input: empty text/terms, repeated terms, and custom tags are handled in highlight/snippet/search paths (T2) | MUST |
| REQ-TH-102 | BM25 defaults are pinned by tests so ranking behavior cannot drift silently (T3) | MUST |
| REQ-TH-103 | Schema field defaults are pinned per type so a field cannot silently lose indexing/storage behavior (T4) | MUST |

## Robustness

| ID | Requirement | Priority |
|----|-------------|----------|
| REQ-TH-200 | Error variants are distinct, `Debug`- and `Display`-formatted for operator diagnostics | SHOULD |
| REQ-TH-201 | `IndexConfig`/`IndexSettings`/`FieldType` are `Debug`-comparable configuration data | SHOULD |

## Traceability Matrix

| Requirement | Test (fn, file) | Property class |
|-------------|-----------------|----------------|
| REQ-TH-001 | `field_def_text_defaults`, `field_def_i64_defaults`, `field_def_u64_defaults`, `field_def_f64_defaults`, `field_def_bool_defaults`, `field_def_date_time_defaults`, `field_def_indexed_only`, `field_def_stored_only`, `field_def_chained_modifiers`, `field_definition_*` (`src/schema.rs` tests, `tests/integration.rs`) | unit |
| REQ-TH-002 | `QueryBuilder`/search unit suite (`src/query.rs`), `search_error_query_display` | unit |
| REQ-TH-003 | `bm25_config_defaults`, `bm25_config_new_matches_default`, `bm25_new_matches_default`, `bm25_default_values`, `bm25_config_builder_chaining`, `bm25_builder_b`, `bm25_builder_k1`, `bm25_builder_field_boosts`, `bm25_builder_recency_boost`, `bm25_builder_full_chain` | unit |
| REQ-TH-004 | `highlighter_default_tags`, `highlighter_default_trait`, `highlighter_custom_tags`, `highlight_custom_tags_star`, `highlighter_with_custom_tags`, `highlight_single_term`, `highlight_multiple_terms`, `highlight_no_match`, `highlight_repeated_term`, `highlight_empty_text`, `highlight_empty_terms`, `snippet_basic`, `snippet_no_match_returns_start`, `snippet_contains_match_near_position`, `snippet_with_custom_tags` | unit |
| REQ-TH-005 | `index_config_with_fields`, `index_settings_defaults`, engine lifecycle tests (`src/lib.rs`, `tests/integration.rs`) | unit/integration |
| REQ-TH-006 | `search_error_variants_distinct`, `search_error_schema_display`, `search_error_query_display`, `search_error_index_display`, `search_error_io_display`, `search_error_debug_format` | unit |
| REQ-TH-100 | Typed-builder query construction suite (`src/query.rs`) | design/unit |
| REQ-TH-101 | `highlight_empty_text`, `highlight_empty_terms`, `highlight_no_match`, `highlight_repeated_term`, `snippet_no_match_returns_start` | unit |
| REQ-TH-102 | `bm25_config_new_matches_default`, `bm25_new_matches_default`, `bm25_default_values` | unit |
| REQ-TH-103 | `field_def_*_defaults` suite | unit |
| REQ-TH-200 | `search_error_variants_distinct`, `search_error_debug_format`, `search_error_*_display` | unit |
| REQ-TH-201 | `field_type_debug_format`, `field_type_equality_and_inequality`, `index_config_debug_format` (`tests/proptest.rs`) | unit/property |

## Test Count

- 74 `#[test]` functions across unit, integration, and property suites.
- All-features suite passes with 0 failures; no-default-features suite passes.
