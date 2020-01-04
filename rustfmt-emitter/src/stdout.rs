use std::io::Write;

use rustfmt_config::Verbosity;

use super::*;

#[derive(Debug)]
pub struct StdoutEmitter {
    verbosity: Verbosity,
}

impl StdoutEmitter {
    pub fn new(verbosity: Verbosity) -> Self {
        Self { verbosity }
    }
}

impl Emitter for StdoutEmitter {
    fn emit_formatted_file(
        &mut self,
        output: &mut dyn Write,
        FormattedFile {
            filename,
            formatted_text,
            ..
        }: FormattedFile<'_>,
    ) -> Result<EmitterResult, io::Error> {
        if self.verbosity != Verbosity::Quiet {
            writeln!(output, "{}:\n", filename)?;
        }
        write!(output, "{}", formatted_text)?;
        Ok(EmitterResult::default())
    }
}
