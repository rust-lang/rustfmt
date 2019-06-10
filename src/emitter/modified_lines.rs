use super::*;
use crate::rustfmt_diff::{make_diff, ModifiedLines};
use std::io::Write;

pub(crate) struct ModifiedLinesEmitter<'a, W> {
    out: &'a mut W,
}

impl<'a, W> ModifiedLinesEmitter<'a, W> {
    pub(crate) fn new(out: &'a mut W) -> Self {
        Self { out }
    }
}

impl<'a, W> Emitter for ModifiedLinesEmitter<'a, W>
where
    W: Write,
{
    fn write_file(
        &mut self,
        FormattedFile {
            formatted_text,
            original_text,
            ..
        }: FormattedFile<'_>,
    ) -> Result<bool, io::Error> {
        let mismatch = make_diff(original_text, formatted_text, 0);
        let has_diff = !mismatch.is_empty();
        write!(self.out, "{}", ModifiedLines::from(mismatch))?;
        Ok(has_diff)
    }
}
