use super::*;
use crate::config::Verbosity;
use crate::NewlineStyle;
use std::io::Write;

#[derive(Debug)]
pub(crate) struct StdoutEmitter {
    verbosity: Verbosity,
}

impl StdoutEmitter {
    pub(crate) fn new(verbosity: Verbosity) -> Self {
        Self { verbosity }
    }
}

impl Emitter for StdoutEmitter {
    fn emit_formatted_file_with_line_style(
        &mut self,
        output: &mut dyn Write,
        FormattedFile {
            filename,
            formatted_text,
            ..
        }: FormattedFile<'_>,
        _: NewlineStyle,
    ) -> Result<EmitterResult, io::Error> {
        if self.verbosity != Verbosity::Quiet {
            writeln!(output, "{}:\n", filename)?;
        }
        write!(output, "{}", formatted_text)?;
        Ok(EmitterResult::default())
    }
}
