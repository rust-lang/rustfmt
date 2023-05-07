use super::*;
use crate::rustfmt_diff::{make_diff, ModifiedLines};
use crate::NewlineStyle;
use std::io::Write;

#[derive(Debug, Default)]
pub(crate) struct ModifiedLinesEmitter;

impl Emitter for ModifiedLinesEmitter {
    fn emit_formatted_file_with_line_style(
        &mut self,
        output: &mut dyn Write,
        FormattedFile {
            original_text,
            formatted_text,
            ..
        }: FormattedFile<'_>,
        _: NewlineStyle,
    ) -> Result<EmitterResult, io::Error> {
        const CONTEXT_SIZE: usize = 0;
        let mismatch = make_diff(original_text, formatted_text, CONTEXT_SIZE);
        let has_diff = !mismatch.is_empty();
        write!(output, "{}", ModifiedLines::from(mismatch))?;
        Ok(EmitterResult { has_diff })
    }
}
