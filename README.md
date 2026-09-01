# tantivy-helper

Full-text search for Rust — Tantivy wrapper with BM25 ranking, autocomplete, highlighting, and typed queries.

## Features

- **BM25 ranking** — Configurable k1/b parameters and field boosts
- **Autocomplete** — Phrase prefix queries for search-as-you-type
- **Highlighting** — Custom tag highlighter for result snippets
- **Typed queries** — Term, phrase, prefix, fuzzy, and boolean queries
- **Schema DSL** — Fluent field definition API

## Schema Definition

```rust
use tantivy_ext::{IndexConfig, FieldDefinition, IndexSettings};

let config = IndexConfig {
    fields: vec![
        FieldDefinition::text("title").fast(true),
        FieldDefinition::text("body"),
        FieldDefinition::u64("timestamp").fast(true),
        FieldDefinition::bool("published"),
    ],
    tokenizers: vec!["default".into()],
    settings: IndexSettings {
        index_base_path: "./my-index".into(),
        ..Default::default()
    },
};
```

## Search Example

```rust
use tantivy_ext::{SearchEngine, QueryBuilder};

let engine = SearchEngine::new(&config)?;
let query = QueryBuilder::new(engine.index(), engine.schema())
    .term("title", "rust")?
    .prefix("body", "search")?
    .build()?;

let results = engine.search(&*query, 10)?;
for hit in &results {
    println!("score={} doc={:?}", hit.score, hit.doc_address);
}
```

## Comparison with raw tantivy

| Feature | tantivy-helper | raw tantivy |
|---|---|---|
| Schema definition | Fluent builder | Manual SchemaBuilder |
| Query building | Typed builder | Manual query construction |
| BM25 config | Structured config | Direct parameter tuning |
| Highlighting | Built-in with tags | Manual implementation |
| Error handling | Unified `SearchError` | Multiple error types |

## License

Licensed under either of [MIT](LICENSE-MIT) or [Apache-2.0](LICENSE-APACHE) at your option.
