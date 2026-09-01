/// Errors that can occur in search operations.
#[derive(Debug, thiserror::Error)]
pub enum SearchError {
    /// Index error.
    #[error("index error: {0}")]
    Index(String),

    /// Query error.
    #[error("query error: {0}")]
    Query(String),

    /// Schema error.
    #[error("schema error: {0}")]
    Schema(String),

    /// I/O error.
    #[error("io error: {0}")]
    Io(#[from] std::io::Error),
}
