#![deny(rust_2018_idioms)]
#![warn(unreachable_pub)]
#![feature(cell_leak)]

#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate log;

use std::path::PathBuf;

use rustc_ast::ast;

pub use crate::config::{
    load_config, CliOptions, Config, Edition, FileLines, FileName, NewlineStyle, Range,
};
pub use crate::emitter::rustfmt_diff::{ModifiedChunk, ModifiedLines};
pub use crate::format_report_formatter::{FormatReportFormatter, FormatReportFormatterBuilder};
pub use crate::formatting::report::{FormatReport, FormatResult};

use crate::formatting::format_input_inner;
use crate::{
    emitter::{Color, Verbosity},
    result::{ErrorKind, FormatError, OperationError},
};

#[cfg(feature = "config")]
pub mod config;
#[cfg(feature = "emitter")]
pub mod emitter;

mod format_report_formatter;
mod formatting;
mod release_channel;
pub mod result;

#[cfg(test)]
mod test;

/// Configures how rustfmt operates during formatting.
#[derive(Clone, Copy, Default)]
pub struct OperationSetting {
    /// If set to `true`, format sub-modules which are defined in the given input.
    pub recursive: bool,
    pub verbosity: Verbosity,
}

/// The main entry point for Rustfmt. Formats the given input according to the
/// given config. `out` is only necessary if required by the configuration.
pub fn format(
    input: Input,
    config: &Config,
    operation_setting: OperationSetting,
) -> Result<FormatReport, OperationError> {
    format_input_inner(input, config, operation_setting)
}

pub fn format_inputs<'a>(
    inputs: impl Iterator<Item = (Input, &'a Config)>,
    operation_setting: OperationSetting,
) -> Result<FormatReport, OperationError> {
    let mut format_report = FormatReport::new();
    for (input, config) in inputs {
        let report = format(input, config, operation_setting)?;
        format_report.merge(report);
    }
    Ok(format_report)
}

/// The input to rustfmt.
#[derive(Debug)]
pub enum Input {
    /// A file on the filesystem.
    File(PathBuf),
    /// A UTF-8 string, in many cases from stdin.
    Text(String),
}

impl Input {
    fn file_name(&self) -> FileName {
        match *self {
            Input::File(ref file) => FileName::Real(file.clone()),
            Input::Text(..) => FileName::Stdin,
        }
    }

    fn to_directory_ownership(&self) -> Option<DirectoryOwnership> {
        match self {
            Input::File(ref file) => {
                // If there exists a directory with the same name as an input,
                // then the input should be parsed as a sub module.
                let file_stem = file.file_stem()?;
                if file.parent()?.to_path_buf().join(file_stem).is_dir() {
                    Some(DirectoryOwnership::Owned {
                        relative: file_stem.to_str().map(symbol::Ident::from_str),
                    })
                } else {
                    None
                }
            }
            _ => None,
        }
    }
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