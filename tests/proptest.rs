//! Property-based tests for tantivy-helper crate.

use proptest::prelude::*;

use tantivy_helper::{FieldDefinition, FieldType};

fn arb_field_type() -> impl Strategy<Value = FieldType> {
    prop_oneof![
        Just(FieldType::Text),
        Just(FieldType::U64),
        Just(FieldType::I64),
        Just(FieldType::F64),
        Just(FieldType::DateTime),
        Just(FieldType::Bool),
    ]
}

proptest! {
    #[test]
    fn field_definition_name_preserved(name in "[a-z]{1,50}") {
        let f = FieldDefinition::text(&name);
        prop_assert_eq!(f.name, name);
    }

    #[test]
    fn field_definition_type_always_valid(ft in arb_field_type()) {
        let f = match ft {
            FieldType::Text => FieldDefinition::text("test"),
            FieldType::U64 => FieldDefinition::u64("test"),
            FieldType::I64 => FieldDefinition::i64("test"),
            FieldType::F64 => FieldDefinition::f64("test"),
            FieldType::DateTime => FieldDefinition::date_time("test"),
            FieldType::Bool => FieldDefinition::bool("test"),
        };
        prop_assert_eq!(f.field_type, ft);
    }

    #[test]
    fn field_definition_chained_stored(
        stored in proptest::bool::ANY,
    ) {
        let f = FieldDefinition::text("test").stored(stored);
        prop_assert_eq!(f.stored, stored);
    }

    #[test]
    fn field_definition_chained_indexed(
        indexed in proptest::bool::ANY,
    ) {
        let f = FieldDefinition::text("test").indexed(indexed);
        prop_assert_eq!(f.indexed, indexed);
    }

    #[test]
    fn field_definition_chained_fast(
        fast in proptest::bool::ANY,
    ) {
        let f = FieldDefinition::u64("test").fast(fast);
        prop_assert_eq!(f.fast, fast);
    }

    #[test]
    fn field_type_equality_reflexive(ft in arb_field_type()) {
        prop_assert_eq!(ft.clone(), ft);
    }

    #[test]
    fn field_type_debug_always_non_empty(ft in arb_field_type()) {
        let debug = format!("{:?}", ft);
        prop_assert!(!debug.is_empty());
    }
}
