use super::*;
use crate::config::Verbosity;
use std::io::Write;

pub(crate) struct StdoutEmitter<'a, W> {
    out: &'a mut W,
    verbosity: Verbosity,
}

impl<'a, W> StdoutEmitter<'a, W> {
    pub(crate) fn new(out: &'a mut W, verbosity: Verbosity) -> Self {
        Self { out, verbosity }
    }
}

impl<'a, W> Emitter for StdoutEmitter<'a, W>
where
    W: Write,
{
    fn write_file(
        &mut self,
        FormattedFile {
            formatted_text,
            filename,
            ..
        }: FormattedFile<'_>,
    ) -> Result<bool, io::Error> {
        if self.verbosity != Verbosity::Quiet {
            println!("{}:\n", filename);
        }
        write!(self.out, "{}", formatted_text)?;
        Ok(false)
    }
}
