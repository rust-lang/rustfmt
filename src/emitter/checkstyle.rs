use super::*;
use crate::checkstyle::output_checkstyle_file;
use crate::rustfmt_diff::make_diff;
use std::io::Write;

pub(crate) struct CheckstyleEmitter<'a, W> {
    out: &'a mut W,
}

impl<'a, W> CheckstyleEmitter<'a, W> {
    pub(crate) fn new(out: &'a mut W) -> Self {
        Self { out }
    }
}

impl<'a, W> Emitter for CheckstyleEmitter<'a, W>
where
    W: Write,
{
    fn write_file(
        &mut self,
        FormattedFile {
            formatted_text,
            original_text,
            filename,
            ..
        }: FormattedFile<'_>,
    ) -> Result<bool, io::Error> {
        let filename = ensure_real_path(filename);
        let diff = make_diff(original_text, formatted_text, 3);
        output_checkstyle_file(&mut self.out, filename, diff)?;
        Ok(false)
    }
}
