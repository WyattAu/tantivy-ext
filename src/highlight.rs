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
    ///
    /// The snippet window is at most [`Highlighter::max_tokens`] tokens
    /// (whitespace-separated), so the knob bounds the returned size.
    pub fn snippet(&self, text: &str, query_terms: &[String]) -> String {
        let lower_text = text.to_lowercase();
        let lower_terms: Vec<String> = query_terms.iter().map(|t| t.to_lowercase()).collect();

        let pos = lower_terms
            .iter()
            .find_map(|term| lower_text.find(term.as_str()))
            .unwrap_or(0);

        let start = pos.saturating_sub(40);
        let end = (pos + 120).min(lower_text.len());

        let window = map_window_to_text(text, &lower_text, start, end);
        let highlighted = self.highlight(window, query_terms);
        limit_tokens(&highlighted, self.max_tokens)
    }
}

/// Byte window `[start, end)` located in the case-folded copy, mapped back
/// onto the original text.
///
/// Case folding can change byte lengths (e.g. `İ` lowercases to two chars),
/// so offsets found in the folded copy are translated through char indices.
/// The resulting slice is always on char boundaries — slicing the original
/// text at raw folded offsets could panic on non-ASCII input.
fn map_window_to_text<'a>(text: &'a str, folded: &str, start: usize, end: usize) -> &'a str {
    if text.chars().count() == folded.chars().count() {
        let start_char = folded[..start].chars().count();
        let end_char = folded[..end].chars().count();
        let text_start = text
            .char_indices()
            .nth(start_char)
            .map_or(text.len(), |(i, _)| i);
        let text_end = text
            .char_indices()
            .nth(end_char)
            .map_or(text.len(), |(i, _)| i);
        &text[text_start..text_end]
    } else {
        // Char counts diverge under case folding: char-index translation is
        // impossible, so fall back to a boundary-safe prefix window.
        let mut start_b = start.min(text.len());
        while start_b > 0 && !text.is_char_boundary(start_b) {
            start_b -= 1;
        }
        let mut end_b = end.min(text.len()).max(start_b);
        while end_b < text.len() && !text.is_char_boundary(end_b) {
            end_b += 1;
        }
        &text[start_b..end_b]
    }
}

/// Cap `s` at `max_tokens` whitespace-separated tokens.
fn limit_tokens(s: &str, max_tokens: usize) -> String {
    let mut kept = s.split_whitespace().take(max_tokens).peekable();
    if kept.peek().is_none() {
        return String::new();
    }
    kept.collect::<Vec<_>>().join(" ")
}

impl Default for Highlighter {
    fn default() -> Self {
        Self::new()
    }
}
