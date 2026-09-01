use std::path::Path;

use tantivy::collector::TopDocs;
use tantivy::query::Query as TantivyQuery;
use tantivy::schema::*;
use tantivy::{TantivyDocument, Index, IndexWriter, ReloadPolicy};

use crate::error::SearchError;
use crate::schema::IndexConfig;

/// Result of a single search hit.
#[derive(Debug, Clone)]
pub struct SearchResult {
    /// Document address in the index.
    pub doc_address: tantivy::DocAddress,
    /// Relevance score.
    pub score: f32,
    /// Optional highlighted snippet.
    pub snippet: Option<String>,
}

/// Search engine backed by Tantivy.
pub struct SearchEngine {
    index: Index,
    schema: Schema,
    writer: IndexWriter<TantivyDocument>,
}

impl SearchEngine {
    /// Create a new search engine from an index configuration.
    pub fn new(config: &IndexConfig) -> Result<Self, SearchError> {
        let mut schema_builder = Schema::builder();

        for field_def in &config.fields {
            let field = match field_def.field_type {
                crate::schema::FieldType::Text => {
                    let opts = TextOptions::default();
                    let opts = if field_def.stored {
                        opts.set_stored()
                    } else {
                        opts
                    };
                    let opts = if field_def.indexed {
                        opts.set_indexing_options(
                            TextFieldIndexing::default().set_tokenizer("default"),
                        )
                    } else {
                        opts
                    };
                    schema_builder.add_text_field(&field_def.name, opts)
                }
                crate::schema::FieldType::U64 => {
                    let opts = NumericOptions::default();
                    let opts = if field_def.stored {
                        opts.set_stored()
                    } else {
                        opts
                    };
                    let opts = if field_def.fast {
                        opts.set_fast()
                    } else {
                        opts
                    };
                    let opts = if field_def.indexed {
                        opts.set_indexed()
                    } else {
                        opts
                    };
                    schema_builder.add_u64_field(&field_def.name, opts)
                }
                crate::schema::FieldType::I64 => {
                    let opts = NumericOptions::default();
                    let opts = if field_def.stored {
                        opts.set_stored()
                    } else {
                        opts
                    };
                    let opts = if field_def.fast {
                        opts.set_fast()
                    } else {
                        opts
                    };
                    let opts = if field_def.indexed {
                        opts.set_indexed()
                    } else {
                        opts
                    };
                    schema_builder.add_i64_field(&field_def.name, opts)
                }
                crate::schema::FieldType::F64 => {
                    let opts = NumericOptions::default();
                    let opts = if field_def.stored {
                        opts.set_stored()
                    } else {
                        opts
                    };
                    let opts = if field_def.fast {
                        opts.set_fast()
                    } else {
                        opts
                    };
                    let opts = if field_def.indexed {
                        opts.set_indexed()
                    } else {
                        opts
                    };
                    schema_builder.add_f64_field(&field_def.name, opts)
                }
                crate::schema::FieldType::DateTime => {
                    let opts = DateOptions::default();
                    let opts = if field_def.stored {
                        opts.set_stored()
                    } else {
                        opts
                    };
                    let opts = if field_def.fast {
                        opts.set_fast()
                    } else {
                        opts
                    };
                    let opts = if field_def.indexed {
                        opts.set_indexed()
                    } else {
                        opts
                    };
                    schema_builder.add_date_field(&field_def.name, opts)
                }
                crate::schema::FieldType::Bool => {
                    let opts = NumericOptions::default();
                    let opts = if field_def.stored {
                        opts.set_stored()
                    } else {
                        opts
                    };
                    let opts = if field_def.indexed {
                        opts.set_indexed()
                    } else {
                        opts
                    };
                    schema_builder.add_bool_field(&field_def.name, opts)
                }
            };
            let _ = field;
        }

        let schema = schema_builder.build();
        let index_path = Path::new(&config.settings.index_base_path);

        std::fs::create_dir_all(index_path)?;

        let index = Index::builder()
            .schema(schema.clone())
            .create_in_dir(index_path)
            .map_err(|e| SearchError::Index(e.to_string()))?;

        let writer = index
            .writer(config.settings.num_threads)
            .map_err(|e| SearchError::Index(e.to_string()))?;

        Ok(Self {
            index,
            schema,
            writer,
        })
    }

    /// Index a single document.
    pub fn index_document(
        &mut self,
        doc: TantivyDocument,
    ) -> Result<(), SearchError> {
        self.writer
            .add_document(doc)
            .map_err(|e| SearchError::Index(e.to_string()))?;
        Ok(())
    }

    /// Commit all pending changes to the index.
    pub fn commit(&mut self) -> Result<(), SearchError> {
        self.writer
            .commit()
            .map_err(|e| SearchError::Index(e.to_string()))?;
        Ok(())
    }

    /// Execute a search query and return ranked results.
    pub fn search(
        &self,
        query: &dyn TantivyQuery,
        limit: usize,
    ) -> Result<Vec<SearchResult>, SearchError> {
        let searcher = self.index.reader_builder()
            .reload_policy(ReloadPolicy::OnCommitWithDelay)
            .try_into()
            .map_err(|e| SearchError::Index(e.to_string()))?
            .searcher();

        let top_docs = searcher
            .search(query, &TopDocs::with_limit(limit))
            .map_err(|e| SearchError::Query(e.to_string()))?;

        let results = top_docs
            .into_iter()
            .map(|(score, doc_addr)| SearchResult {
                doc_address: doc_addr,
                score,
                snippet: None,
            })
            .collect();

        Ok(results)
    }

    /// Delete a document by term match.
    pub fn delete(&mut self, field: &str, value: &str) -> Result<u64, SearchError> {
        let field_entry = self.schema.get_field(field)
            .map_err(|_| SearchError::Schema(format!("field '{field}' not found")))?;

        let term = Term::from_field_text(field_entry, value);
        let count = self.writer
            .delete_term(term);
        Ok(count)
    }

    /// Get a reference to the index schema.
    pub fn schema(&self) -> &Schema {
        &self.schema
    }

    /// Get a reference to the underlying Tantivy index.
    pub fn index(&self) -> &Index {
        &self.index
    }
}
