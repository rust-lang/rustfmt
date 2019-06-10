use super::*;
use crate::checkstyle::{footer, header, output_checkstyle_file};
use crate::rustfmt_diff::make_diff;
use std::io::Write;

pub(crate) struct CheckstyleEmitter;

impl CheckstyleEmitter {
    pub(crate) fn new() -> Self {
        Self
    }
}

impl<W> Emitter<W> for CheckstyleEmitter
where
    W: Write,
{
    fn emit_header(&self, output: &mut W) -> Result<(), io::Error> {
        write!(output, "{}", header())
    }

    fn emit_footer(&self, output: &mut W) -> Result<(), io::Error> {
        write!(output, "{}", footer())
    }

    fn emit_formatted_file(
        &mut self,
        output: &mut W,
        FormattedFile {
            formatted_text,
            original_text,
            filename,
            ..
        }: FormattedFile<'_>,
    ) -> Result<bool, io::Error> {
        let filename = ensure_real_path(filename);
        let diff = make_diff(original_text, formatted_text, 3);
        output_checkstyle_file(output, filename, diff)?;
        Ok(false)
    }
}
