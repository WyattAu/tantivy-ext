use serde::{Deserialize, Serialize};

/// BM25 ranking configuration.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BM25Config {
    /// Term frequency saturation parameter (default: 1.2).
    pub k1: f32,
    /// Length normalization parameter (default: 0.75).
    pub b: f32,
    /// Per-field boost weights.
    pub field_boosts: Vec<FieldBoost>,
    /// Recency boost decay factor (0.0 = disabled).
    pub recency_boost: f32,
}

impl Default for BM25Config {
    fn default() -> Self {
        Self {
            k1: 1.2,
            b: 0.75,
            field_boosts: Vec::new(),
            recency_boost: 0.0,
        }
    }
}

/// Boost weight for a specific field.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FieldBoost {
    /// Field name.
    pub field: String,
    /// Boost weight.
    pub weight: f32,
}

impl BM25Config {
    /// Create a new BM25 configuration with default values.
    pub fn new() -> Self {
        Self::default()
    }

    /// Set the term frequency saturation parameter.
    pub fn k1(mut self, k1: f32) -> Self {
        self.k1 = k1;
        self
    }

    /// Set the length normalization parameter.
    pub fn b(mut self, b: f32) -> Self {
        self.b = b;
        self
    }

    /// Add a field boost weight.
    pub fn field_boost(mut self, field: impl Into<String>, weight: f32) -> Self {
        self.field_boosts.push(FieldBoost {
            field: field.into(),
            weight,
        });
        self
    }

    /// Set the recency boost decay factor.
    pub fn recency_boost(mut self, decay: f32) -> Self {
        self.recency_boost = decay;
        self
    }
}
