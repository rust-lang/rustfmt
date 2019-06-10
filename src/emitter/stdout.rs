use super::*;
use crate::config::Verbosity;
use std::io::Write;

pub(crate) struct StdoutEmitter {
    verbosity: Verbosity,
}

impl StdoutEmitter {
    pub(crate) fn new(verbosity: Verbosity) -> Self {
        Self { verbosity }
    }
}

impl<W> Emitter<W> for StdoutEmitter
where
    W: Write,
{
    fn emit_formatted_file(
        &self,
        output: &mut W,
        FormattedFile {
            formatted_text,
            filename,
            ..
        }: FormattedFile<'_>,
    ) -> Result<bool, io::Error> {
        if self.verbosity != Verbosity::Quiet {
            println!("{}:\n", filename);
        }
        write!(output, "{}", formatted_text)?;
        Ok(false)
    }
}
