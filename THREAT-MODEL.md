# Threat Model — tantivy-ext

Status: **v1.0** · Method: STRIDE over the public API surface
(`SearchEngine`, `QueryBuilder`, `IndexConfig`, `FieldDefinition`,
`BM25Config`, `Highlighter`, `SearchResult`, `SearchError`).

Trust boundaries: (1) end-user query input (terms, prefixes, fuzzy
parameters) flowing into the builder, (2) indexed documents (attacker
may control indexed text), (3) the tantivy engine and its query
language underneath, (4) HTML output contexts consuming `highlight`
output.

## Assets

| ID | Asset | Example |
|----|-------|---------|
| A1 | Query-language containment | User text interpreted as tantivy query syntax (boolean operators, field selectors) enabling query injection |
| A2 | Availability of the search path | Malformed or hostile queries panicking the engine |
| A3 | Result fidelity | Boost/recency misconfiguration silently re-ranking results |
| A4 | Safe embedding of highlighted text | Highlight tags injected into raw document text creating HTML/XSS issues downstream |

## STRIDE Analysis

| # | Threat | Category | Surface | Mitigation | Verifying test |
|---|--------|----------|---------|------------|----------------|
| T1 | Query injection via raw user text | Elevation | `QueryBuilder::term`/`prefix`/`fuzzy` | Queries are built through the typed builder API over tantivy's programmatic query objects — user text never reaches tantivy's query *parser*; all query construction tests go through typed fields | `QueryBuilder` unit suite; `search_error_query_display` |
| T2 | Panic on hostile input (empty text, unknown terms, empty highlight set) | DoS | `highlight`, `snippet`, `search` | Highlight/snippet paths handle empty text, no-match, repeated terms, and custom tags without panicking; search errors are typed `SearchError`s | `highlight_empty_text`, `highlight_empty_terms`, `highlight_no_match`, `highlight_repeated_term`, `highlight_single_term`, `snippet_no_match_returns_start`, `search_error_variants_distinct` |
| T3 | Ranking drift via unvalidated BM25 tuning | Tampering | `BM25Config`, `FieldBoost`, `recency_boost` | BM25 parameters (`b`, `k1`, boosts, recency) are explicit builder values with defaults pinned by tests so silent default drift is detectable | `bm25_config_defaults`, `bm25_config_new_matches_default`, `bm25_new_matches_default`, `bm25_default_values`, `bm25_builder_full_chain`, `bm25_builder_b`, `bm25_builder_k1`, `bm25_builder_field_boosts`, `bm25_builder_recency_boost` |
| T4 | Schema drift: fields silently mis-typed or unindexed | Tampering | `FieldDefinition`, `FieldType` | Typed field constructors (`text`, `i64`, `u64`, `f64`, `bool`, `date_time`) with explicit `indexed`/`stored`/`fast` modifiers; defaults pinned per type | `field_def_text_defaults`, `field_def_i64_defaults`, `field_def_u64_defaults`, `field_def_f64_defaults`, `field_def_bool_defaults`, `field_def_date_time_defaults`, `field_def_indexed_only`, `field_def_stored_only`, `field_def_chained_modifiers` |
| T5 | Malformed highlight markup breaks host HTML | Tampering | `Highlighter` | Highlight tags are caller-supplied and applied by wrapping matched terms only — the crate performs no HTML escaping itself; documented contract, see OPEN-1 | `highlighter_default_tags`, `highlighter_custom_tags`, `highlight_custom_tags_star`, `highlighter_with_custom_tags` |
| T6 | Snippet windows slicing UTF-8 or missing matches | Tampering | `snippet` | Snippet returns a window anchored on the match position; no-match returns the start position deterministically | `snippet_contains_match_near_position`, `snippet_no_match_returns_start`, `snippet_with_custom_tags` |

## OPEN RISKS (missing mitigations — not fabricated)

- **OPEN-1 — no HTML escaping in highlight output.** `highlight`
  inserts caller-provided tags around matched terms verbatim. Callers
  rendering to HTML must escape document text themselves (or use the
  crate only in non-HTML contexts). This is documented API behavior.
- **OPEN-2 — fuzzy edit distance is caller-set.** Aggressive fuzzy
  settings widen recall (and potential information exposure across
  tenants sharing an index); no guardrail is imposed.

## Out of Scope

- Tantivy-internal security (memory-mapped index files, concurrent
  writers) — owned by tantivy.
- Authorization of *which* documents a querier may see.
- Query cost/complexity limits (e.g. regex-style DoS) beyond what
  tantivy enforces natively.

## Residual Risks

- Hostile *indexed content* can contain adversarial text (homoglyphs,
  huge terms); ranking quality degrades but memory safety is tantivy's
  contract.
