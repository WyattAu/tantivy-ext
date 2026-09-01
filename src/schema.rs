use serde::{Deserialize, Serialize};

/// Configuration for a search index.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IndexConfig {
    /// Field definitions.
    pub fields: Vec<FieldDefinition>,
    /// Custom tokenizer names.
    pub tokenizers: Vec<String>,
    /// Index settings.
    pub settings: IndexSettings,
}

/// Settings for the search index.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IndexSettings {
    /// Number of indexing threads.
    pub num_threads: usize,
    /// Temporary directory for index construction.
    pub temp_directory: Option<String>,
    /// Base path for persistent index storage.
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
    /// Field name.
    pub name: String,
    /// Field data type.
    pub field_type: FieldType,
    /// Whether the field value is stored in the index.
    pub stored: bool,
    /// Whether the field is searchable.
    pub indexed: bool,
    /// Whether the field is available for fast access.
    pub fast: bool,
}

/// Supported field types in the index schema.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum FieldType {
    /// Full-text field.
    Text,
    /// Unsigned 64-bit integer.
    U64,
    /// Signed 64-bit integer.
    I64,
    /// 64-bit floating point.
    F64,
    /// Date and time.
    DateTime,
    /// Boolean value.
    Bool,
}

impl FieldDefinition {
    /// Create a text field definition.
    pub fn text(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            field_type: FieldType::Text,
            stored: true,
            indexed: true,
            fast: false,
        }
    }

    /// Create a u64 field definition.
    pub fn u64(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            field_type: FieldType::U64,
            stored: true,
            indexed: false,
            fast: true,
        }
    }

    /// Create an i64 field definition.
    pub fn i64(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            field_type: FieldType::I64,
            stored: true,
            indexed: false,
            fast: true,
        }
    }

    /// Create an f64 field definition.
    pub fn f64(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            field_type: FieldType::F64,
            stored: true,
            indexed: false,
            fast: true,
        }
    }

    /// Create a date-time field definition.
    pub fn date_time(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            field_type: FieldType::DateTime,
            stored: true,
            indexed: false,
            fast: true,
        }
    }

    /// Create a boolean field definition.
    pub fn bool(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            field_type: FieldType::Bool,
            stored: true,
            indexed: false,
            fast: true,
        }
    }

    /// Set whether the field is stored.
    pub fn stored(mut self, stored: bool) -> Self {
        self.stored = stored;
        self
    }

    /// Set whether the field is indexed.
    pub fn indexed(mut self, indexed: bool) -> Self {
        self.indexed = indexed;
        self
    }

    /// Set whether the field is available for fast access.
    pub fn fast(mut self, fast: bool) -> Self {
        self.fast = fast;
        self
    }
}
