use crate::buf_println;
use super::*;
use crate::config::Verbosity;

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
    fn emit_formatted_file(
        &mut self,
        _output: &mut dyn Write,
        printer: &Printer,
        FormattedFile {
            filename,
            formatted_text,
            ..
        }: FormattedFile<'_>,
    ) -> Result<EmitterResult, io::Error> {
        if self.verbosity != Verbosity::Quiet {
            buf_println!(printer, "{filename}:\n");
        }
        buf_println!(printer, "{formatted_text}");
        Ok(EmitterResult::default())
    }
}
