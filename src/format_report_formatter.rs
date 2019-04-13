use crate::config::FileName;
use crate::formatting::FormattingError;
use crate::{ErrorKind, FormatReport};
use annotate_snippets::display_list::DisplayList;
use annotate_snippets::formatter::DisplayListFormatter;
use annotate_snippets::snippet::{Annotation, AnnotationType, Slice, Snippet, SourceAnnotation};
use std::fmt::{self, Display};

pub struct ReportFormatterBuilder<'a> {
    report: &'a FormatReport,
    enable_colors: bool,
}

impl<'a> ReportFormatterBuilder<'a> {
    pub fn new(report: &'a FormatReport) -> Self {
        Self {
            report,
            enable_colors: false,
        }
    }

    pub fn enable_colors(self, enable_colors: bool) -> Self {
        Self {
            enable_colors,
            ..self
        }
    }

    pub fn build(self) -> ReportFormatter<'a> {
        ReportFormatter {
            report: self.report,
            enable_colors: self.enable_colors,
        }
    }
}

pub struct ReportFormatter<'a> {
    report: &'a FormatReport,
    enable_colors: bool,
}

impl<'a> Display for ReportFormatter<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let formatter = DisplayListFormatter::new(self.enable_colors);
        let errors_by_file = &self.report.internal.borrow().0;

        for (file, errors) in errors_by_file {
            for error in errors {
                let snippet = formatting_error_to_snippet(file, error);
                writeln!(f, "{}", formatter.format(&DisplayList::from(snippet)))?;
                writeln!(f, "")?;
            }
        }

        if !errors_by_file.is_empty() {
            let snippet = Snippet {
                title: Some(Annotation {
                    id: None,
                    label: Some(format!(
                        "rustfmt may have failed to format. See previous {} errors.",
                        self.report.warning_count()
                    )),
                    annotation_type: AnnotationType::Warning,
                }),
                footer: Vec::new(),
                slices: Vec::new(),
            };
            writeln!(f, "{}", formatter.format(&DisplayList::from(snippet)))?;
        }

        Ok(())
    }
}

fn formatting_error_to_snippet(file: &FileName, error: &FormattingError) -> Snippet {
    let (space_len, target_len) = error.format_len();
    let annotation_type = error_kind_to_snippet_annotation_type(&error.kind);
    let slice_annotations = if target_len > 0 {
        vec![SourceAnnotation {
            range: (space_len, space_len + target_len),
            label: String::new(),
            annotation_type: AnnotationType::Error,
        }]
    } else {
        Vec::new()
    };

    let title_annotation_id = if error.is_internal() {
        Some("internal".to_string())
    } else {
        None
    };

    Snippet {
        title: Some(Annotation {
            id: title_annotation_id,
            label: Some(format!("{}", error.kind)),
            annotation_type,
        }),
        footer: vec![Annotation {
            id: None,
            label: Some(error.msg_suffix().to_string()),
            annotation_type: AnnotationType::Note,
        }],
        slices: vec![Slice {
            source: error.line_buffer.clone(),
            line_start: error.line,
            origin: Some(format!("{}:{}", file, error.line)),
            fold: false,
            annotations: slice_annotations,
        }],
    }
}

fn error_kind_to_snippet_annotation_type(error_kind: &ErrorKind) -> AnnotationType {
    match error_kind {
        ErrorKind::LineOverflow(..)
        | ErrorKind::TrailingWhitespace
        | ErrorKind::IoError(_)
        | ErrorKind::ParseError
        | ErrorKind::LostComment
        | ErrorKind::LicenseCheck
        | ErrorKind::BadAttr
        | ErrorKind::VersionMismatch => AnnotationType::Error,
        ErrorKind::BadIssue(_) | ErrorKind::DeprecatedAttr => AnnotationType::Warning,
    }
}
