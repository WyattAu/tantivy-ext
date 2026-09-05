#![no_main]

use libfuzzer_sys::fuzz_target;
use tantivy::schema::{Schema, TEXT};
use tantivy::Index;
use tantivy_helper::QueryBuilder;

fuzz_target!(|data: &[u8]| {
    // Bound input so query construction stays fast.
    let s = String::from_utf8_lossy(&data[..data.len().min(4096)]);

    // In-memory index with two text fields; both the found and "field not
    // found" Err paths are exercised by the arbitrary field names below.
    let mut builder = Schema::builder();
    builder.add_text_field("title", TEXT);
    builder.add_text_field("body", TEXT);
    let schema = builder.build();
    let index = Index::create_in_ram(schema.clone());

    // Split input into field / value / prefix thirds at char boundaries.
    let mut a = s.len() / 3;
    let mut b = 2 * s.len() / 3;
    while a > 0 && !s.is_char_boundary(a) {
        a -= 1;
    }
    while b > 0 && !s.is_char_boundary(b) {
        b -= 1;
    }
    let (field, rest) = s.split_at(a);
    let (value, prefix) = rest.split_at(b - a);

    // Every builder step is Result and must return Err, never panic.
    let qb = QueryBuilder::new(&index, &schema);
    let qb = match qb.term(field, value) {
        Ok(q) => q,
        Err(_) => return,
    };
    let qb = match qb.term("title", &s) {
        Ok(q) => q,
        Err(_) => return,
    };
    let qb = match qb.fuzzy(field, value, u8::from(data.first().copied().unwrap_or(0)) % 3) {
        Ok(q) => q,
        Err(_) => return,
    };
    // Multi-term path exercises the BooleanQuery branch in `build`.
    match qb.prefix(field, prefix) {
        Ok(qb) => {
            let _ = qb.build();
        }
        Err(_) => return,
    }

    // Single-term path exercises the single-query branch in `build`.
    let qb = QueryBuilder::new(&index, &schema);
    if let Ok(qb) = qb.term(field, &s) {
        let _ = qb.build();
    }

    // Empty prefix and unknown field are documented Err paths.
    let qb = QueryBuilder::new(&index, &schema);
    let _ = qb.prefix(field, "");
    let qb = QueryBuilder::new(&index, &schema);
    let _ = qb.build();
});
