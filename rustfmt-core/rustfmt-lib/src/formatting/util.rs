use crate::{config::Config, emitter::Verbosity, Input, NewlineStyle, OperationSetting};
use crate::formatting::{
    comment::LineClasses,
    FormattedSnippet,
    shape::Indent,
    utils::indent_next_line,
};

/// Format the given snippet. The snippet is expected to be *complete* code.
/// When we cannot parse the given snippet, this function returns `None`.
pub(crate) fn format_snippet(snippet: &str, config: &Config) -> Option<FormattedSnippet> {
    let mut config = config.clone();
    std::panic::catch_unwind(move || {
        config.set().hide_parse_errors(true);

        let result = {
            let input = Input::Text(snippet.into());
            crate::format(
                input,
                &config,
                OperationSetting {
                    verbosity: Verbosity::Quiet,
                    ..OperationSetting::default()
                },
            )
        };
        match result {
            Ok(report) if !report.has_errors() => {
                match report.format_result_as_rc().borrow().iter().next() {
                    Some((_, format_result))
                        if format_result.all_errors().count() == 0
                            && !format_result.formatted_text().is_empty() =>
                    {
                        Some(format_result.formatted_snippet().clone())
                    }
                    _ => None,
                }
            }
            _ => None,
        }
    })
    // Discard panics encountered while formatting the snippet
    // The ? operator is needed to remove the extra Option
    .ok()?
}

/// Format the given code block. Mainly targeted for code block in comment.
/// The code block may be incomplete (i.e., parser may be unable to parse it).
/// To avoid panic in parser, we wrap the code block with a dummy function.
/// The returned code block does **not** end with newline.
pub(crate) fn format_code_block(code_snippet: &str, config: &Config) -> Option<FormattedSnippet> {
    const FN_MAIN_PREFIX: &str = "fn main() {\n";

    fn enclose_in_main_block(s: &str, config: &Config) -> String {
        let indent = Indent::from_width(config, config.tab_spaces());
        let mut result = String::with_capacity(s.len() * 2);
        result.push_str(FN_MAIN_PREFIX);
        let mut need_indent = true;
        for (kind, line) in LineClasses::new(s) {
            if need_indent {
                result.push_str(&indent.to_string(config));
            }
            result.push_str(&line);
            result.push('\n');
            need_indent = indent_next_line(kind);
        }
        result.push('}');
        result
    }

    // Wrap the given code block with `fn main()` if it does not have one.
    let snippet = enclose_in_main_block(code_snippet, config);
    let mut result = String::with_capacity(snippet.len());
    let mut is_first = true;

    // While formatting the code, ignore the config's newline style setting and always use "\n"
    // instead of "\r\n" for the newline characters. This is ok because the output here is
    // not directly outputted by rustfmt command, but used by the comment formatter's input.
    // We have output-file-wide "\n" ==> "\r\n" conversion process after here if it's necessary.
    let mut config_with_unix_newline = config.clone();
    config_with_unix_newline
        .set()
        .newline_style(NewlineStyle::Unix);
    let mut formatted = format_snippet(&snippet, &config_with_unix_newline)?;
    // Remove wrapping main block
    formatted.unwrap_code_block();

    // Trim "fn main() {" on the first line and "}" on the last line,
    // then unindent the whole code block.
    let block_len = formatted
        .snippet
        .rfind('}')
        .unwrap_or_else(|| formatted.snippet.len());
    let mut is_indented = true;
    let indent_str = Indent::from_width(config, config.tab_spaces()).to_string(config);
    for (kind, ref line) in LineClasses::new(&formatted.snippet[FN_MAIN_PREFIX.len()..block_len]) {
        if !is_first {
            result.push('\n');
        } else {
            is_first = false;
        }
        let trimmed_line = if !is_indented {
            line
        } else if line.len() > config.max_width() {
            // If there are lines that are larger than max width, we cannot tell
            // whether we have succeeded but have some comments or strings that
            // are too long, or we have failed to format code block. We will be
            // conservative and just return `None` in this case.
            return None;
        } else if line.len() > indent_str.len() {
            // Make sure that the line has leading whitespaces.
            if line.starts_with(indent_str.as_ref()) {
                let offset = if config.hard_tabs() {
                    1
                } else {
                    config.tab_spaces()
                };
                &line[offset..]
            } else {
                line
            }
        } else {
            line
        };
        result.push_str(trimmed_line);
        is_indented = indent_next_line(kind);
    }
    Some(FormattedSnippet {
        snippet: result,
        non_formatted_ranges: formatted.non_formatted_ranges,
    })
}

#[cfg(test)]
mod unit_tests {
    use super::*;

    #[test]
    fn test_no_panic_on_format_snippet_and_format_code_block() {
        // `format_snippet()` and `format_code_block()` should not panic
        // even when we cannot parse the given snippet.
        let snippet = "let";
        assert!(format_snippet(snippet, &Config::default()).is_none());
        assert!(format_code_block(snippet, &Config::default()).is_none());
    }

    fn test_format_inner<F>(formatter: F, input: &str, expected: &str) -> bool
    where
        F: Fn(&str, &Config) -> Option<FormattedSnippet>,
    {
        let output = formatter(input, &Config::default());
        output.is_some() && output.unwrap().snippet == expected
    }

    #[test]
    fn test_format_snippet() {
        let snippet = "fn main() { println!(\"hello, world\"); }";
        #[cfg(not(windows))]
        let expected = "fn main() {\n    \
                        println!(\"hello, world\");\n\
                        }\n";
        #[cfg(windows)]
        let expected = "fn main() {\r\n    \
                        println!(\"hello, world\");\r\n\
                        }\r\n";
        assert!(test_format_inner(format_snippet, snippet, expected));
    }

    #[test]
    fn test_format_code_block_fail() {
        #[rustfmt::skip]
        let code_block = "this_line_is_100_characters_long_xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx(x, y, z);";
        assert!(format_code_block(code_block, &Config::default()).is_none());
    }

    #[test]
    fn test_format_code_block() {
        // simple code block
        let code_block = "let x=3;";
        let expected = "let x = 3;";
        assert!(test_format_inner(format_code_block, code_block, expected));

        // more complex code block, taken from chains.rs.
        let code_block =
"let (nested_shape, extend) = if !parent_rewrite_contains_newline && is_continuable(&parent) {
(
chain_indent(context, shape.add_offset(parent_rewrite.len())),
context.config.indent_style() == IndentStyle::Visual || is_small_parent,
)
} else if is_block_expr(context, &parent, &parent_rewrite) {
match context.config.indent_style() {
// Try to put the first child on the same line with parent's last line
IndentStyle::Block => (parent_shape.block_indent(context.config.tab_spaces()), true),
// The parent is a block, so align the rest of the chain with the closing
// brace.
IndentStyle::Visual => (parent_shape, false),
}
} else {
(
chain_indent(context, shape.add_offset(parent_rewrite.len())),
false,
)
};
";
        let expected =
"let (nested_shape, extend) = if !parent_rewrite_contains_newline && is_continuable(&parent) {
    (
        chain_indent(context, shape.add_offset(parent_rewrite.len())),
        context.config.indent_style() == IndentStyle::Visual || is_small_parent,
    )
} else if is_block_expr(context, &parent, &parent_rewrite) {
    match context.config.indent_style() {
        // Try to put the first child on the same line with parent's last line
        IndentStyle::Block => (parent_shape.block_indent(context.config.tab_spaces()), true),
        // The parent is a block, so align the rest of the chain with the closing
        // brace.
        IndentStyle::Visual => (parent_shape, false),
    }
} else {
    (
        chain_indent(context, shape.add_offset(parent_rewrite.len())),
        false,
    )
};";
        assert!(test_format_inner(format_code_block, code_block, expected));
    }
}
