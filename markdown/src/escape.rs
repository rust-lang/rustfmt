use super::MarkdownFormatter;

const ATX_HEADER_ESCAPES: [&'static str; 6] = ["# ", "## ", "### ", "#### ", "##### ", "###### "];

impl<'i, F> MarkdownFormatter<'i, F> {
    pub(super) fn needs_escape(&mut self, input: &str) -> bool {
        if !self.last_was_softbreak {
            // We _should_ only need to escape after a softbreak since the markdown formatter will
            // adjust the indentation. Depending on the context we'll either remove leading spaces
            // or add indentation (spaces or '>') depending on if we're in a list or blockquote.
            // See <https://spec.commonmark.org/0.30/#example-70> as an example where the semantics
            // would change without an escape after removing indentation.
            return false;
        }

        self.last_was_softbreak = false;

        if input.len() <= 2 {
            return false;
        }

        let Some(first_char) = input.chars().next() else {
            return false;
        };

        let is_setext_heading = |value: u8| input.trim_end().bytes().all(|b| b == value);
        let is_unordered_list_marker = |value: &str| input.starts_with(value);
        let is_thematic_break = |value: u8| input.bytes().all(|b| b == value || b == b' ');

        match first_char {
            '#' => ATX_HEADER_ESCAPES
                .iter()
                .any(|header| input.starts_with(header)),
            '=' => is_setext_heading(b'='),
            '-' => {
                is_unordered_list_marker("- ") || is_setext_heading(b'-') || is_thematic_break(b'-')
            }
            '_' => is_thematic_break(b'_'),
            '*' => is_unordered_list_marker("* ") || is_thematic_break(b'*'),
            '+' => is_unordered_list_marker("+ "),
            '>' => true,
            _ => false,
        }
    }
}
