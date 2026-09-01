/// Highlighter for search result snippets.
pub struct Highlighter {
    /// Tag inserted before matched terms.
    pub pre_tag: String,
    /// Tag inserted after matched terms.
    pub post_tag: String,
    /// Maximum tokens in a snippet.
    pub max_tokens: usize,
}

impl Highlighter {
    /// Create a new highlighter with default tags (`<mark>`/`</mark>`).
    pub fn new() -> Self {
        Self {
            pre_tag: "<mark>".into(),
            post_tag: "</mark>".into(),
            max_tokens: 20,
        }
    }

    /// Create a highlighter with custom tags.
    pub fn with_tags(pre: impl Into<String>, post: impl Into<String>) -> Self {
        Self {
            pre_tag: pre.into(),
            post_tag: post.into(),
            ..Self::new()
        }
    }

    /// Highlight matching terms in the text.
    pub fn highlight(&self, text: &str, query_terms: &[String]) -> String {
        let mut result = text.to_string();

        for term in query_terms {
            let highlighted = format!("{}{}{}", self.pre_tag, term, self.post_tag);
            result = result.replace(term, &highlighted);
        }

        result
    }

    /// Extract a snippet from text around the first matching term.
    pub fn snippet(&self, text: &str, query_terms: &[String]) -> String {
        let lower_text = text.to_lowercase();
        let lower_terms: Vec<String> = query_terms.iter().map(|t| t.to_lowercase()).collect();

        let pos = lower_terms
            .iter()
            .find_map(|term| lower_text.find(term.as_str()))
            .unwrap_or(0);

        let start = pos.saturating_sub(40);
        let end = (pos + 120).min(text.len());

        let snippet = &text[start..end];
        self.highlight(snippet, query_terms)
    }
}

impl Default for Highlighter {
    fn default() -> Self {
        Self::new()
    }
}
