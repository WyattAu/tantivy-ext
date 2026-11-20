# Pattern adoption: tantivy-ext

Full-text search extensions for Tantivy. Declared per
`pattern-standards/references/adoption-format.md`.

| Pattern | Status | Evidence |
|---|---|---|
| T13 | adopted | src/schema.rs:1 and src/query.rs:1 (typed schema and query builders stay pure); src/engine.rs:35 owns the index writer and commits as the imperative shell |
| T1 | adopted | src/schema.rs:1 (FieldDefinition/FieldType parse once into typed values; rejection is a typed SearchError, no re-checking) |
| P64 | adopted | tests/proptest.rs:4 (arbitrary FieldType/FieldDefinition strategies drive the suite) |
| P67 | adopted | fuzz/fuzz_targets/fuzz_query.rs:1 (structure-aware fuzzing of query construction) |
