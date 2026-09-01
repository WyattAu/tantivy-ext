use tantivy::query::{BooleanQuery, FuzzyTermQuery, PhrasePrefixQuery, TermQuery};
use tantivy::schema::*;
use tantivy::{Index, Term};

use crate::error::SearchError;

/// Builder for constructing Tantivy queries fluently.
pub struct QueryBuilder<'a> {
    _index: &'a Index,
    schema: &'a Schema,
    terms: Vec<Box<dyn tantivy::query::Query>>,
}

impl<'a> QueryBuilder<'a> {
    /// Create a new query builder.
    pub fn new(index: &'a Index, schema: &'a Schema) -> Self {
        Self {
            _index: index,
            schema,
            terms: Vec::new(),
        }
    }

    /// Add a term-level query for an exact match.
    pub fn term(mut self, field: &str, value: &str) -> Result<Self, SearchError> {
        let field_entry = self
            .schema
            .get_field(field)
            .map_err(|_| SearchError::Schema(format!("field '{field}' not found")))?;
        let term = Term::from_field_text(field_entry, value);
        let query = TermQuery::new(term, tantivy::schema::IndexRecordOption::WithFreqs);
        self.terms.push(Box::new(query));
        Ok(self)
    }

    /// Add a phrase prefix query (autocomplete).
    pub fn prefix(mut self, field: &str, prefix: &str) -> Result<Self, SearchError> {
        let field_entry = self
            .schema
            .get_field(field)
            .map_err(|_| SearchError::Schema(format!("field '{field}' not found")))?;

        let tokens: Vec<Term> = prefix
            .split_whitespace()
            .map(|token| Term::from_field_text(field_entry, token))
            .collect();

        if tokens.is_empty() {
            return Err(SearchError::Query("empty prefix".into()));
        }

        let query = PhrasePrefixQuery::new(tokens);
        self.terms.push(Box::new(query));
        Ok(self)
    }

    /// Add a fuzzy term query for typo-tolerant search.
    pub fn fuzzy(
        mut self,
        field: &str,
        value: &str,
        max_distance: u8,
    ) -> Result<Self, SearchError> {
        let field_entry = self
            .schema
            .get_field(field)
            .map_err(|_| SearchError::Schema(format!("field '{field}' not found")))?;
        let term = Term::from_field_text(field_entry, value);
        let query = FuzzyTermQuery::new(term, max_distance, true);
        self.terms.push(Box::new(query));
        Ok(self)
    }

    /// Combine all accumulated terms with AND logic.
    pub fn build(self) -> Result<Box<dyn tantivy::query::Query>, SearchError> {
        if self.terms.is_empty() {
            return Err(SearchError::Query("no query terms added".into()));
        }

        if self.terms.len() == 1 {
            Ok(self.terms.into_iter().next().expect("checked len == 1"))
        } else {
            let sub: Vec<(tantivy::query::Occur, Box<dyn tantivy::query::Query>)> = self
                .terms
                .into_iter()
                .map(|q| (tantivy::query::Occur::Must, q))
                .collect();
            Ok(Box::new(BooleanQuery::new(sub)))
        }
    }
}
