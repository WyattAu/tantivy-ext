#![forbid(unsafe_code)]
#![deny(missing_docs)]

//! Full-text search engine extensions for Tantivy.

/// Search engine implementation.
pub mod engine;
/// Error types.
pub mod error;
/// Search result highlighting.
pub mod highlight;
/// Query builder for constructing Tantivy queries.
pub mod query;
/// BM25 ranking configuration.
pub mod ranking;
/// Index schema definitions.
pub mod schema;

pub use engine::SearchEngine;
pub use error::SearchError;
pub use query::QueryBuilder;
pub use ranking::BM25Config;
pub use schema::{FieldDefinition, FieldType, IndexConfig};

#[cfg(test)]
mod tests {
    use super::*;
    use crate::highlight::Highlighter;
    use crate::ranking::BM25Config;
    use crate::schema::{FieldDefinition, FieldType, IndexSettings};

    // ---- FieldDefinition tests ----

    #[test]
    fn field_definition_text() {
        let f = FieldDefinition::text("title");
        assert_eq!(f.name, "title");
        assert_eq!(f.field_type, FieldType::Text);
        assert!(f.stored);
        assert!(f.indexed);
        assert!(!f.fast);
    }

    #[test]
    fn field_definition_u64() {
        let f = FieldDefinition::u64("count");
        assert_eq!(f.field_type, FieldType::U64);
        assert!(f.stored);
        assert!(!f.indexed);
        assert!(f.fast);
    }

    #[test]
    fn field_definition_i64() {
        let f = FieldDefinition::i64("timestamp");
        assert_eq!(f.field_type, FieldType::I64);
    }

    #[test]
    fn field_definition_f64() {
        let f = FieldDefinition::f64("score");
        assert_eq!(f.field_type, FieldType::F64);
    }

    #[test]
    fn field_definition_date_time() {
        let f = FieldDefinition::date_time("created_at");
        assert_eq!(f.field_type, FieldType::DateTime);
    }

    #[test]
    fn field_definition_bool() {
        let f = FieldDefinition::bool("active");
        assert_eq!(f.field_type, FieldType::Bool);
    }

    #[test]
    fn field_definition_chained_modifiers() {
        let f = FieldDefinition::text("body").stored(false).indexed(false).fast(true);
        assert!(!f.stored);
        assert!(!f.indexed);
        assert!(f.fast);
    }

    // ---- FieldType enum tests ----

    #[test]
    fn field_type_equality() {
        assert_eq!(FieldType::Text, FieldType::Text);
        assert_eq!(FieldType::U64, FieldType::U64);
        assert_eq!(FieldType::I64, FieldType::I64);
        assert_eq!(FieldType::F64, FieldType::F64);
        assert_eq!(FieldType::DateTime, FieldType::DateTime);
        assert_eq!(FieldType::Bool, FieldType::Bool);
        assert_ne!(FieldType::Text, FieldType::U64);
    }

    #[test]
    fn field_type_debug() {
        assert_eq!(format!("{:?}", FieldType::Text), "Text");
        assert_eq!(format!("{:?}", FieldType::DateTime), "DateTime");
    }

    // ---- BM25Config tests ----

    #[test]
    fn bm25_config_defaults() {
        let cfg = BM25Config::default();
        assert_eq!(cfg.k1, 1.2);
        assert_eq!(cfg.b, 0.75);
        assert!(cfg.field_boosts.is_empty());
        assert_eq!(cfg.recency_boost, 0.0);
    }

    #[test]
    fn bm25_config_new_matches_default() {
        let cfg = BM25Config::new();
        let def = BM25Config::default();
        assert_eq!(cfg.k1, def.k1);
        assert_eq!(cfg.b, def.b);
    }

    #[test]
    fn bm25_config_builder_chaining() {
        let cfg = BM25Config::new()
            .k1(2.0)
            .b(0.5)
            .field_boost("title", 1.5)
            .field_boost("body", 0.8)
            .recency_boost(0.1);

        assert_eq!(cfg.k1, 2.0);
        assert_eq!(cfg.b, 0.5);
        assert_eq!(cfg.field_boosts.len(), 2);
        assert_eq!(cfg.field_boosts[0].field, "title");
        assert_eq!(cfg.field_boosts[0].weight, 1.5);
        assert_eq!(cfg.field_boosts[1].field, "body");
        assert_eq!(cfg.field_boosts[1].weight, 0.8);
        assert_eq!(cfg.recency_boost, 0.1);
    }

    // ---- Highlighter tests ----

    #[test]
    fn highlighter_default_tags() {
        let h = Highlighter::new();
        assert_eq!(h.pre_tag, "<mark>");
        assert_eq!(h.post_tag, "</mark>");
        assert_eq!(h.max_tokens, 20);
    }

    #[test]
    fn highlighter_highlight_single_term() {
        let h = Highlighter::new();
        let result = h.highlight("hello world", &["world".into()]);
        assert_eq!(result, "hello <mark>world</mark>");
    }

    #[test]
    fn highlighter_highlight_multiple_terms() {
        let h = Highlighter::new();
        let result = h.highlight("hello world foo", &["hello".into(), "foo".into()]);
        assert_eq!(result, "<mark>hello</mark> world <mark>foo</mark>");
    }

    #[test]
    fn highlighter_highlight_no_match() {
        let h = Highlighter::new();
        let result = h.highlight("hello world", &["missing".into()]);
        assert_eq!(result, "hello world");
    }

    #[test]
    fn highlighter_with_custom_tags() {
        let h = Highlighter::with_tags("**", "**");
        let result = h.highlight("rust is great", &["rust".into()]);
        assert_eq!(result, "**rust** is great");
    }

    #[test]
    fn highlighter_snippet_basic() {
        let h = Highlighter::new();
        let text = "The quick brown fox jumps over the lazy dog near the river bank on a sunny day";
        let result = h.snippet(text, &["river".into()]);
        assert!(result.contains("<mark>river</mark>"));
    }

    #[test]
    fn highlighter_snippet_no_match_returns_start() {
        let h = Highlighter::new();
        let text = "alpha beta gamma delta";
        let result = h.snippet(text, &["zzz".into()]);
        assert!(result.contains("alpha"));
    }

    // ---- SearchError display tests ----

    #[test]
    fn search_error_index_display() {
        let err = SearchError::Index("write failed".into());
        assert_eq!(err.to_string(), "index error: write failed");
    }

    #[test]
    fn search_error_query_display() {
        let err = SearchError::Query("empty prefix".into());
        assert_eq!(err.to_string(), "query error: empty prefix");
    }

    #[test]
    fn search_error_schema_display() {
        let err = SearchError::Schema("missing field".into());
        assert_eq!(err.to_string(), "schema error: missing field");
    }

    #[test]
    fn search_error_io_display() {
        let io_err = std::io::Error::new(std::io::ErrorKind::PermissionDenied, "denied");
        let err = SearchError::Io(io_err);
        assert_eq!(err.to_string(), "io error: denied");
    }

    // ---- IndexSettings tests ----

    #[test]
    fn index_settings_defaults() {
        let s = IndexSettings::default();
        assert_eq!(s.num_threads, 4);
        assert!(s.temp_directory.is_none());
        assert_eq!(s.index_base_path, "./tantivy-index");
    }
}
