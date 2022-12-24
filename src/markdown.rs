use crate::comment::{hide_sharp_behind_comment, trim_custom_comment_prefix, CodeBlockAttribute};
use crate::Config;
use itertools::Itertools;
use markdown::MarkdownFormatter;
use std::borrow::Cow;

/// Rewrite markdown input.
///
/// The main goal of this function is to reformat rust code blocks in markdown text.
/// However, there will be some light reformatting of other markdown items aside from code blocks.
///
/// **Note:** The content of indented codeblocks will not be formatted, but indentation may change.
pub(crate) fn rewrite_markdown<'a, 'c>(input: &'a str, config: &'c Config) -> Cow<'a, str> {
    let rewrite_rust_code_block = |info_string: &str, code_block_buffer: String| {
        let is_buffer_empty = code_block_buffer.trim().is_empty();
        let is_formattable_code =
            |info_string| CodeBlockAttribute::new(info_string).is_formattable_rust();

        if is_buffer_empty || !is_formattable_code(info_string) {
            return code_block_buffer;
        }

        // First, comment out hidden rustdoc lines as they would prevent us from properly
        // parsing and formatting the code snippet.
        let with_hidden_rustdoc_lines = code_block_buffer
            .lines()
            .map(|line| hide_sharp_behind_comment(line))
            .join("\n");

        if let Some(formatted) =
            crate::format_code_block(&with_hidden_rustdoc_lines, &config, false)
        {
            trim_custom_comment_prefix(&formatted.snippet).into()
        } else {
            trim_custom_comment_prefix(&code_block_buffer).into()
        }
    };

    let formatter = MarkdownFormatter::new(input, rewrite_rust_code_block);
    match formatter.format() {
        Ok(text) => text.into(),
        Err(_) => input.into(),
    }
}
