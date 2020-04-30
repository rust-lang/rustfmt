use crate::config::FileName;
use crate::{ErrorKind, FormatError, FormatReport};
use annotate_snippets::display_list::DisplayList;
use annotate_snippets::formatter::DisplayListFormatter;
use annotate_snippets::snippet::{Annotation, AnnotationType, Slice, Snippet, SourceAnnotation};
use std::fmt::{self, Display};

/// A builder for [`FormatReportFormatter`].
pub struct FormatReportFormatterBuilder<'a> {
    report: &'a FormatReport,
    enable_colors: bool,
}

impl<'a> FormatReportFormatterBuilder<'a> {
    /// Creates a new [`FormatReportFormatterBuilder`].
    pub fn new(report: &'a FormatReport) -> Self {
        Self {
            report,
            enable_colors: false,
        }
    }

    /// Enables colors and formatting in the output.
    pub fn enable_colors(self, enable_colors: bool) -> Self {
        Self {
            enable_colors,
            ..self
        }
    }

    /// Creates a new [`FormatReportFormatter`] from the settings in this builder.
    pub fn build(self) -> FormatReportFormatter<'a> {
        FormatReportFormatter {
            report: self.report,
            enable_colors: self.enable_colors,
        }
    }
}

/// Formats the warnings/errors in a [`FormatReport`].
///
/// Can be created using a [`FormatReportFormatterBuilder`].
pub struct FormatReportFormatter<'a> {
    report: &'a FormatReport,
    enable_colors: bool,
}

impl<'a> Display for FormatReportFormatter<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let formatter = DisplayListFormatter::new(self.enable_colors, false);

        for (file, errors) in self.report.format_result.borrow().iter() {
            for error in errors.errors() {
                let snippet = formatting_error_to_snippet(file, error);
                writeln!(f, "{}\n", formatter.format(&DisplayList::from(snippet)))?;
            }
        }

        if self.report.has_errors() {
            let snippet = formatting_failure_snippet(self.report.warning_count());
            writeln!(f, "{}", formatter.format(&DisplayList::from(snippet)))?;
        }

        Ok(())
    }
}

fn formatting_failure_snippet(warning_count: usize) -> Snippet {
    Snippet {
        title: Some(Annotation {
            id: None,
            label: Some(format!(
                "rustfmt has failed to format. See previous {} errors.",
                warning_count
            )),
            annotation_type: AnnotationType::Warning,
        }),
        slices: Vec::new(),
        footer: vec![],
    }
}

fn formatting_error_to_snippet(file: &FileName, error: &FormatError) -> Snippet {
    let slices = vec![snippet_code_slice(file, error)];
    let title = Some(snippet_title(error));

    Snippet {
        title,
        slices,
        footer: vec![],
    }
}

fn snippet_title(error: &FormatError) -> Annotation {
    let annotation_type = error_kind_to_snippet_annotation_type(&error.kind);

    Annotation {
        id: None,
        label: Some(error.kind.to_string()),
        annotation_type,
    }
}

fn snippet_code_slice(file: &FileName, error: &FormatError) -> Slice {
    let annotations = slice_annotation(error).into_iter().collect();
    let origin = error
        .line_num
        .as_ref()
        .map(|line_num| format!("{}:{}", file, line_num));
    let source = error.line_str.clone().unwrap_or_else(|| String::new());

    Slice {
        source,
        line_start: error.line_num.unwrap_or(0),
        origin,
        fold: false,
        annotations,
    }
}

fn slice_annotation(error: &FormatError) -> Option<SourceAnnotation> {
    match error.format_len() {
        Some((range_start, range_length)) if range_length > 0 => {
            let range_end = range_start + range_length;
            Some(SourceAnnotation {
                annotation_type: AnnotationType::Error,
                range: (range_start, range_end),
                label: String::new(),
            })
        }
        _ => None,
    }
}

fn error_kind_to_snippet_annotation_type(error_kind: &ErrorKind) -> AnnotationType {
    match error_kind {
        ErrorKind::LineOverflow(..)
        | ErrorKind::MacroFormatError
        | ErrorKind::TrailingWhitespace
        | ErrorKind::LicenseCheck
        | ErrorKind::BadAttr => AnnotationType::Error,
        ErrorKind::DeprecatedAttr => AnnotationType::Warning,
    }
}
