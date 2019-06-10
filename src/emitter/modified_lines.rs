use super::*;
use crate::rustfmt_diff::{make_diff, ModifiedLines};
use std::io::Write;

pub(crate) struct ModifiedLinesEmitter;

impl ModifiedLinesEmitter {
    pub(crate) fn new() -> Self {
        Self
    }
}

impl Emitter for ModifiedLinesEmitter {
    fn emit_formatted_file(
        &self,
        output: &mut dyn Write,
        FormattedFile {
            original_text,
            formatted_text,
            ..
        }: FormattedFile<'_>,
    ) -> Result<bool, io::Error> {
        let mismatch = make_diff(original_text, formatted_text, 0);
        let has_diff = !mismatch.is_empty();
        write!(output, "{}", ModifiedLines::from(mismatch))?;
        Ok(has_diff)
    }
}
