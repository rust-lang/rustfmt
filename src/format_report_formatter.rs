use crate::{
    formatting::report::FormatReport,
    result::{ErrorKind, FormatError},
};
use annotate_snippets::display_list::{DisplayList, FormatOptions};
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
        let opt = FormatOptions {
            color: self.enable_colors,
            ..Default::default()
        };
        for (file, errors) in self.report.format_result_as_rc().borrow().iter() {
            for error in errors.errors_excluding_macro() {
                let annotation_type = error_kind_to_snippet_annotation_type(&error.kind());
                let label = error.kind().to_string();
                let title = Annotation {
                    id: None,
                    label: Some(&label),
                    annotation_type,
                };
                let origin = error
                    .line_num()
                    .as_ref()
                    .map(|line_num| format!("{}:{}", file, line_num));
                let slice = Slice {
                    source: error.line_str().unwrap_or(""),
                    line_start: error.line_num().unwrap_or(0),
                    origin: origin.as_deref(),
                    fold: false,
                    annotations: slice_annotation(error).into_iter().collect(),
                };

                let snippet = Snippet {
                    title: Some(title),
                    slices: vec![slice],
                    footer: vec![],
                    opt,
                };
                writeln!(f, "{}\n", DisplayList::from(snippet))?;
            }
        }

        if self.report.has_errors() {
            let label = format!(
                "rustfmt has failed to format. See previous {} errors.",
                self.report.warning_count()
            );
            let snippet = Snippet {
                title: Some(Annotation {
                    id: None,
                    label: Some(&label),
                    annotation_type: AnnotationType::Warning,
                }),
                slices: Vec::new(),
                footer: vec![],
                opt,
            };
            writeln!(f, "{}", DisplayList::from(snippet))?;
        }

        Ok(())
    }
}

fn slice_annotation(error: &FormatError) -> Option<SourceAnnotation<'static>> {
    match error.format_len() {
        Some((range_start, range_length)) if range_length > 0 => {
            let range_end = range_start + range_length;
            Some(SourceAnnotation {
                annotation_type: AnnotationType::Error,
                range: (range_start, range_end),
                label: "",
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
