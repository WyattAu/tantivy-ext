# Changelog

All notable changes to this project are documented here. Format: [Keep a
Changelog](https://keepachangelog.com/) — versions follow [semver](https://semver.org).

## [0.3.0] - 2026-09-12

### Removed (dead config knobs — config-matrix audit)

- **`BM25Config` / `FieldBoost` (entire `ranking` module) and the `bm25`
  feature.** The struct was settable and even unit-tested as a setter, but
  nothing ever read it: the engine used tantivy's built-in BM25 scoring and
  tantivy 0.26 does not expose k1/b for configuration. The README claimed
  "Configurable k1/b parameters and field boosts" — that was never true.
  A config knob that cannot observably change behavior is a bug; wiring it
  would require replacing tantivy's scoring pipeline (a feature, not a
  fix), so the struct is removed.
- **`IndexConfig::tokenizers`** — never read; the engine hardcodes the
  `default` tokenizer for text fields and no tokenizer registration API
  exists.
- **`IndexSettings::temp_directory`** — never read by the engine.

### Fixed

- **`IndexSettings::num_threads` was not just inert — it broke engine
  construction.** tantivy 0.26 changed `Index::writer(n)` to take a memory
  budget in bytes, not a thread count, so `SearchEngine::new` failed for
  every realistic value ("memory arena ... at least 15000000"). The engine
  now uses `writer_with_num_threads` with a valid per-thread budget.
- **`FieldDefinition::fast` was silently dropped for `Text` and `Bool`
  fields.** It now reaches the tantivy schema for every field type.
- **`Highlighter::max_tokens` was settable but never read.** `snippet()`
  now caps its output at `max_tokens` tokens.
- **`Highlighter::snippet` could panic on non-ASCII input** (byte offsets
  from the case-folded copy sliced the original text off char boundaries;
  `İ` even changes length under folding). Window mapping is now
  char-boundary safe.

### Added

- `tests/config_matrix.rs`: behavior-observable test for every remaining
  public knob (with_tags, max_tokens, index_base_path, num_threads,
  stored, indexed, fast, term, prefix, fuzzy, build), per the estate
  config-matrix standard.

## [0.2.0] - 2026-09-05

### Added
- Initial public release.
