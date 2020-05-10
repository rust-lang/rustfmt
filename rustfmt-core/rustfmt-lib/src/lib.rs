#![deny(rust_2018_idioms)]
#![warn(unreachable_pub)]
#![feature(cell_leak)]

#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate log;

use std::cell::{Ref, RefCell};
use std::collections::{BTreeMap, BTreeSet, HashSet};
use std::io;
use std::panic;
use std::path::PathBuf;
use std::rc::Rc;

use rustc_ast::ast;
use rustc_span::{symbol, Span};
use thiserror::Error;

pub use crate::config::{
    load_config, CliOptions, Config, Edition, FileLines, FileName, NewlineStyle, Range,
};
pub use crate::emitter::rustfmt_diff::{ModifiedChunk, ModifiedLines};
pub use crate::format_report_formatter::{FormatReportFormatter, FormatReportFormatterBuilder};

use crate::formatting::format_input_inner;
use crate::{
    comment::LineClasses,
    emitter::{Color, Verbosity},
    shape::Indent,
    syntux::parser::DirectoryOwnership,
    syntux::session::ParseSess,
    utils::indent_next_line,
};

#[macro_use]
mod utils;

#[cfg(feature = "config")]
pub mod config;
#[cfg(feature = "emitter")]
pub mod emitter;

mod attr;
mod chains;
mod closures;
mod comment;
mod expr;
mod format_report_formatter;
pub(crate) mod formatting;
mod ignore_path;
mod imports;
mod items;
mod lists;
mod macros;
mod matches;
mod missed_spans;
pub(crate) mod modules;
mod overflow;
mod pairs;
mod patterns;
mod release_channel;
mod reorder;
mod rewrite;
mod shape;
mod skip;
pub(crate) mod source_map;
mod spanned;
mod stmt;
mod string;
mod syntux;
#[cfg(test)]
mod test;
mod types;
mod vertical;
pub(crate) mod visitor;

/// The various errors that can occur during formatting.
#[derive(Error, Clone, Debug, Hash, Eq, PartialEq)]
pub enum ErrorKind {
    /// Line has exceeded character limit (found, maximum).
    #[error(
        "line formatted, but exceeded maximum width \
         (maximum: {1} (see `max_width` option), found: {0})"
    )]
    LineOverflow(usize, usize),
    /// Line ends in whitespace.
    #[error("left behind trailing whitespace")]
    TrailingWhitespace,
    /// License check has failed.
    #[error("license check failed")]
    LicenseCheck,
    /// Used deprecated skip attribute.
    #[error("`rustfmt_skip` is deprecated; use `rustfmt::skip`")]
    DeprecatedAttr,
    /// Used a rustfmt:: attribute other than skip or skip::macros.
    #[error("invalid attribute")]
    BadAttr,
    /// Failed to format macro calls.
    #[error("failed to format macro calls")]
    MacroFormatError,
}

#[derive(Error, Clone, Debug, Hash, Eq, PartialEq)]
#[error("{kind}")]
pub struct FormatError {
    kind: ErrorKind,
    line_num: Option<usize>,
    line_str: Option<String>,
}

impl FormatError {
    pub(crate) fn new(kind: ErrorKind, line_num: usize, line_str: String) -> Self {
        FormatError {
            kind,
            line_num: Some(line_num),
            line_str: Some(line_str),
        }
    }

    pub(crate) fn err_without_line_info(kind: ErrorKind) -> Self {
        FormatError {
            kind,
            line_num: None,
            line_str: None,
        }
    }

    pub(crate) fn from_span(kind: ErrorKind, parse_sess: &ParseSess, span: Span) -> Self {
        FormatError {
            kind,
            line_num: Some(parse_sess.line_of_byte_pos(span.lo())),
            line_str: Some(parse_sess.span_to_first_line_string(span)),
        }
    }

    // (space, target)
    pub(crate) fn format_len(&self) -> Option<(usize, usize)> {
        match self.kind {
            ErrorKind::LineOverflow(found, max) => Some((max, found - max)),
            ErrorKind::TrailingWhitespace
            | ErrorKind::DeprecatedAttr
            | ErrorKind::BadAttr
            | ErrorKind::LicenseCheck => {
                let len = self.line_str.as_ref().map_or(0, |s| s.len());
                let trailing_ws_start = self
                    .line_str
                    .as_ref()
                    .and_then(|s| s.rfind(|c: char| !c.is_whitespace()))
                    .map(|pos| pos + 1)
                    .unwrap_or(0);
                Some((trailing_ws_start, len - trailing_ws_start))
            }
            _ => None,
        }
    }
}

/// Result of formatting a snippet of code along with ranges of lines that didn't get formatted,
/// i.e., that got returned as they were originally.
#[derive(Debug, Clone, Default)]
struct FormattedSnippet {
    snippet: String,
    non_formatted_ranges: Vec<NonFormattedRange>,
}

impl FormattedSnippet {
    /// In case the snippet needed to be wrapped in a function, this shifts down the ranges of
    /// non-formatted code.
    fn unwrap_code_block(&mut self) {
        self.non_formatted_ranges.iter_mut().for_each(|range| {
            *range = range.shift_up();
        });
    }

    /// Returns `true` if the line n did not get formatted.
    fn is_line_non_formatted(&self, n: usize) -> bool {
        self.non_formatted_ranges
            .iter()
            .any(|range| range.contains(n))
    }
}

/// The result of formatting, including the formatted text and various
/// errors and warning arose while formatting.
#[derive(Debug, Clone, Default)]
pub struct FormatResult {
    original_snippet: Option<String>,
    formatted_snippet: FormattedSnippet,
    format_errors: HashSet<FormatError>,
    newline_style: NewlineStyle,
}

#[derive(Debug, Clone, Copy)]
struct NonFormattedRange {
    lo: usize,
    hi: usize,
}

impl NonFormattedRange {
    pub(crate) fn new(lo: usize, hi: usize) -> NonFormattedRange {
        NonFormattedRange { lo, hi }
    }

    fn shift_up(self) -> NonFormattedRange {
        NonFormattedRange {
            lo: self.lo - 1,
            hi: self.hi - 1,
        }
    }

    fn contains(&self, line: usize) -> bool {
        self.lo <= line && line <= self.hi
    }
}

#[derive(Error, Debug)]
pub enum OperationError {
    /// The user mandated a version and the current version of rustfmt does not
    /// satisfy that requirement.
    #[error("version mismatch")]
    VersionMismatch,
    /// An io error during reading or writing.
    #[error("io error: {0}")]
    IoError(io::Error),
    /// Invalid glob pattern in `ignore` configuration option.
    #[error("invalid glob pattern found in ignore list: {0}")]
    InvalidGlobPattern(ignore::Error),
    /// Parse error occurred while parsing the input.
    #[error("failed to parse {input:?}")]
    ParseError { input: FileName, is_panic: bool },
}

impl OperationError {
    #[cfg(test)]
    pub fn is_parse_error(&self) -> bool {
        match self {
            OperationError::ParseError { .. } => true,
            _ => false,
        }
    }
}

impl FormatResult {
    pub(crate) fn success(
        snippet: String,
        non_formatted_ranges: Vec<NonFormattedRange>,
        original_snippet: Option<String>,
        newline_style: NewlineStyle,
    ) -> Self {
        let formatted_snippet = FormattedSnippet {
            snippet,
            non_formatted_ranges,
        };
        FormatResult {
            original_snippet,
            formatted_snippet,
            format_errors: HashSet::new(),
            newline_style,
        }
    }

    pub(crate) fn errors(&self) -> impl Iterator<Item = &FormatError> {
        self.format_errors
            .iter()
            .filter(|e| e.kind != ErrorKind::MacroFormatError)
    }
}

/// Reports on any issues that occurred during a run of Rustfmt.
///
/// Can be reported to the user using the `Display` impl on [`FormatReportFormatter`].
#[derive(Debug, Clone)]
pub struct FormatReport {
    format_result: Rc<RefCell<BTreeMap<FileName, FormatResult>>>,
    ignored_files: Rc<RefCell<BTreeSet<FileName>>>,
}

impl FormatReport {
    pub fn new() -> FormatReport {
        FormatReport {
            format_result: Rc::new(RefCell::new(BTreeMap::new())),
            ignored_files: Rc::new(RefCell::new(BTreeSet::new())),
        }
    }

    pub fn format_result(&self) -> impl Iterator<Item = (&FileName, &FormatResult)> {
        Ref::leak(RefCell::borrow(&self.format_result)).iter()
    }

    /// FIXME(topecongiro): reduce visibility.
    pub fn merge(&mut self, other: Self) {
        self.format_result
            .borrow_mut()
            .append(&mut other.format_result.borrow_mut());
        self.ignored_files
            .borrow_mut()
            .append(&mut other.ignored_files.borrow_mut());
    }

    pub(crate) fn add_ignored_file(&self, file_name: FileName) {
        self.ignored_files.borrow_mut().insert(file_name);
    }

    fn add_format_result(&self, file_name: FileName, format_result: FormatResult) {
        let mut format_results = self.format_result.borrow_mut();
        let mut original_format_result = format_results.entry(file_name).or_default();
        original_format_result.formatted_snippet = format_result.formatted_snippet;
        original_format_result
            .format_errors
            .extend(format_result.format_errors);
        if original_format_result.original_snippet.is_none() {
            original_format_result.original_snippet = format_result.original_snippet;
        }
    }

    fn append_errors(&self, f: FileName, errors: impl Iterator<Item = FormatError>) {
        let mut format_result = self.format_result.borrow_mut();
        let format_errors = &mut format_result.entry(f).or_default().format_errors;
        for err in errors {
            format_errors.insert(err);
        }
    }

    pub(crate) fn add_macro_format_failure(&self, file_name: FileName) {
        self.add_format_error(
            file_name,
            FormatError::err_without_line_info(ErrorKind::MacroFormatError),
        );
    }

    pub(crate) fn add_license_failure(&self, file_name: FileName) {
        self.add_format_error(
            file_name,
            FormatError::err_without_line_info(ErrorKind::LicenseCheck),
        );
    }

    pub(crate) fn add_format_error(&self, file_name: FileName, format_error: FormatError) {
        self.format_result
            .borrow_mut()
            .entry(file_name)
            .or_default()
            .format_errors
            .insert(format_error);
    }

    pub fn has_errors(&self) -> bool {
        RefCell::borrow(&self.format_result)
            .iter()
            .any(|(_, format_result)| format_result.errors().count() > 0)
    }

    fn warning_count(&self) -> usize {
        RefCell::borrow(&self.format_result)
            .iter()
            .map(|(_, format_result)| format_result.errors().count())
            .sum()
    }

    /// Whether any warnings or errors are present in the report.
    pub fn has_warnings(&self) -> bool {
        self.has_errors()
    }
}

/// Format the given snippet. The snippet is expected to be *complete* code.
/// When we cannot parse the given snippet, this function returns `None`.
fn format_snippet(snippet: &str, config: &Config) -> Option<FormattedSnippet> {
    let mut config = config.clone();
    panic::catch_unwind(move || {
        config.set().hide_parse_errors(true);

        let result = {
            let input = Input::Text(snippet.into());
            format(
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
                match (*report.format_result)
                    .clone()
                    .into_inner()
                    .into_iter()
                    .next()
                {
                    Some((
                        _,
                        FormatResult {
                            formatted_snippet,
                            format_errors,
                            ..
                        },
                    )) if format_errors.is_empty() && !formatted_snippet.snippet.is_empty() => {
                        Some(formatted_snippet)
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
fn format_code_block(code_snippet: &str, config: &Config) -> Option<FormattedSnippet> {
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

#[derive(Debug)]
pub enum Input {
    File(PathBuf),
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
