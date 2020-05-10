use std::cell::{Ref, RefCell};
use std::collections::{BTreeMap, BTreeSet, HashSet};
use std::rc::Rc;

use crate::formatting::FormattedSnippet;
use crate::result::{ErrorKind, FormatError};
use crate::FileName;
use crate::NewlineStyle;

/// Reports on any issues that occurred during a run of Rustfmt.
///
/// Can be reported to the user using the `Display` impl on [`FormatReportFormatter`].
#[derive(Debug, Clone)]
pub struct FormatReport {
    format_result: Rc<RefCell<BTreeMap<FileName, FormatResult>>>,
    ignored_files: Rc<RefCell<BTreeSet<FileName>>>,
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

/// The inclusive range of the input which was not formatted, represented by a pair of line numbers.
#[derive(Debug, Clone, Copy)]
pub(crate) struct NonFormattedRange {
    lo: usize,
    hi: usize,
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

    pub(crate) fn all_errors(&self) -> impl Iterator<Item = &FormatError> {
        self.format_errors.iter()
    }

    pub(crate) fn errors_excluding_macro(&self) -> impl Iterator<Item = &FormatError> {
        self.format_errors
            .iter()
            .filter(|e| e.kind() != ErrorKind::MacroFormatError)
    }

    /// Return the newline style used to format the result.
    pub fn newline_style(&self) -> NewlineStyle {
        self.newline_style.clone()
    }

    pub fn original_text(&self) -> Option<&str> {
        self.original_snippet.as_ref().map(|s| s.as_str())
    }

    pub fn formatted_text(&self) -> &str {
        &self.formatted_snippet.snippet
    }

    pub(crate) fn formatted_snippet(&self) -> &FormattedSnippet {
        &self.formatted_snippet
    }
}

impl FormatReport {
    pub fn new() -> FormatReport {
        FormatReport {
            format_result: Rc::new(RefCell::new(BTreeMap::new())),
            ignored_files: Rc::new(RefCell::new(BTreeSet::new())),
        }
    }

    /// Returns the result of formatting the given input, including the formatted text and
    /// various warnings and errors encountered during formatting.
    //
    // NOTE: Avoid using this method internally: use `format_result_as_rc` instead.
    pub fn format_result(&self) -> impl Iterator<Item = (&FileName, &FormatResult)> {
        Ref::leak(RefCell::borrow(&self.format_result)).iter()
    }

    pub(crate) fn format_result_as_rc(&self) -> Rc<RefCell<BTreeMap<FileName, FormatResult>>> {
        Rc::clone(&self.format_result)
    }

    pub(crate) fn merge(&mut self, other: Self) {
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

    pub(crate) fn add_format_result(&self, file_name: FileName, format_result: FormatResult) {
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

    pub(crate) fn append_errors(&self, f: FileName, errors: impl Iterator<Item = FormatError>) {
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
            .any(|(_, format_result)| format_result.errors_excluding_macro().count() > 0)
    }

    pub(crate) fn warning_count(&self) -> usize {
        RefCell::borrow(&self.format_result)
            .iter()
            .map(|(_, format_result)| format_result.errors_excluding_macro().count())
            .sum()
    }

    /// Whether any warnings or errors are present in the report.
    pub fn has_warnings(&self) -> bool {
        self.has_errors()
    }
}

impl NonFormattedRange {
    pub(crate) fn new(lo: usize, hi: usize) -> NonFormattedRange {
        NonFormattedRange { lo, hi }
    }

    pub(crate) fn shift_up(self) -> NonFormattedRange {
        NonFormattedRange {
            lo: self.lo - 1,
            hi: self.hi - 1,
        }
    }

    pub(crate) fn contains(&self, line: usize) -> bool {
        self.lo <= line && line <= self.hi
    }
}
