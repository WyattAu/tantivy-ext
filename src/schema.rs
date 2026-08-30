use serde::{Deserialize, Serialize};

/// Configuration for a search index.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IndexConfig {
    pub fields: Vec<FieldDefinition>,
    pub tokenizers: Vec<String>,
    pub settings: IndexSettings,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IndexSettings {
    pub num_threads: usize,
    pub temp_directory: Option<String>,
    pub index_base_path: String,
}

impl Default for IndexSettings {
    fn default() -> Self {
        Self {
            num_threads: 4,
            temp_directory: None,
            index_base_path: "./tantivy-index".into(),
        }
    }
}

/// Definition of a single index field.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FieldDefinition {
    pub name: String,
    pub field_type: FieldType,
    pub stored: bool,
    pub indexed: bool,
    pub fast: bool,
}

/// Supported field types in the index schema.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum FieldType {
    Text,
    U64,
    I64,
    F64,
    DateTime,
    Bool,
}

impl FieldDefinition {
    pub fn text(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            field_type: FieldType::Text,
            stored: true,
            indexed: true,
            fast: false,
        }
    }

    pub fn u64(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            field_type: FieldType::U64,
            stored: true,
            indexed: false,
            fast: true,
        }
    }

    pub fn i64(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            field_type: FieldType::I64,
            stored: true,
            indexed: false,
            fast: true,
        }
    }

    pub fn f64(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            field_type: FieldType::F64,
            stored: true,
            indexed: false,
            fast: true,
        }
    }

    pub fn date_time(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            field_type: FieldType::DateTime,
            stored: true,
            indexed: false,
            fast: true,
        }
    }

    pub fn bool(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            field_type: FieldType::Bool,
            stored: true,
            indexed: false,
            fast: true,
        }
    }

    pub fn stored(mut self, stored: bool) -> Self {
        self.stored = stored;
        self
    }

    pub fn indexed(mut self, indexed: bool) -> Self {
        self.indexed = indexed;
        self
    }

    pub fn fast(mut self, fast: bool) -> Self {
        self.fast = fast;
        self
    }
}
