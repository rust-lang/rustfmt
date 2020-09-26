//! This module defines errors and functions related to working with `Result`.

use rustc_span::Span;
use thiserror::Error;

use crate::formatting::modules::ModuleResolutionError;
use crate::{formatting::ParseSess, FileName};

/// Represents the specific error kind of [`FormatError`].
#[derive(Error, Clone, Copy, Debug, Hash, Eq, PartialEq)]
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

/// Represents errors related to formatting issues.
#[derive(Error, Clone, Debug, Hash, Eq, PartialEq)]
#[error("{kind}")]
pub struct FormatError {
    kind: ErrorKind,
    line_num: Option<usize>,
    line_str: Option<String>,
}

impl FormatError {
    /// Return the specific kind of this error.
    pub fn kind(&self) -> ErrorKind {
        self.kind
    }

    /// Return the line number on which this error arose.
    pub fn line_num(&self) -> Option<usize> {
        self.line_num
    }

    /// Return the content of the line on which this error arose.
    pub fn line_str(&self) -> Option<&str> {
        self.line_str.as_deref()
    }

    pub(crate) fn new(kind: ErrorKind, line_num: usize, line_str: String) -> Self {
        FormatError {
            kind,
            line_num: Some(line_num),
            line_str: Some(line_str),
        }
    }

    // FIXME(topecongiro): add line information in every case.
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

/// Represent errors unrelated to formatting issues.
#[derive(Error, Debug)]
pub enum OperationError {
    /// The user mandated a version and the current version of rustfmt does not
    /// satisfy that requirement.
    #[error("version mismatch")]
    VersionMismatch,
    /// Error during module resolution.
    #[error("{0}")]
    ModuleResolutionError(#[from] ModuleResolutionError),
    /// Invalid glob pattern in `ignore` configuration option.
    #[error("invalid glob pattern found in ignore list: {0}")]
    InvalidGlobPattern(ignore::Error),
    /// Parse error occurred while parsing the input.
    #[error("failed to parse {input}")]
    ParseError { input: FileName, is_panic: bool },
    /// Io error.
    #[error("{0}")]
    IoError(#[from] std::io::Error),
}

impl OperationError {
    #[cfg(test)]
    pub fn is_parse_error(&self) -> bool {
        matches!(self, OperationError::ParseError { .. })
    }
}
