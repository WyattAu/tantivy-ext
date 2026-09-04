# tantivy-helper

Full-text search for Rust — Tantivy wrapper with BM25 ranking, autocomplete, highlighting, and typed queries.

Built on **tantivy 0.26** (v0.2.0). MSRV is 1.86.

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

## Version history

- **0.2.0** — Bump to tantivy 0.26. Public API unchanged except:
  - `SearchEngine::search` now ranks via `TopDocs::with_limit(limit).order_by_score()` (tantivy 0.26 made `TopDocs` a collector blueprint, not a `Collector` itself). Behavior is identical: score-ranked top-K.
  - MSRV raised to 1.86 (tantivy 0.26 requirement).
- **0.1.0** — Initial release on tantivy 0.22.

## Ecosystem alignment

This crate and [kestrel](https://github.com/WyattAu/kestrel) are aligned on tantivy 0.26. Kestrel currently uses raw tantivy directly; migrating its `IndexService` onto `SearchEngine`/`QueryBuilder` is future work.

## License

Licensed under either of [MIT](LICENSE-MIT) or [Apache-2.0](LICENSE-APACHE) at your option.
