use super::*;
use crate::config::Verbosity;

#[derive(Debug)]
pub(crate) struct IntoOutputEmitter {
    verbosity: Verbosity,
}

impl IntoOutputEmitter {
    pub(crate) fn new(verbosity: Verbosity) -> Self {
        Self { verbosity }
    }
}

impl Emitter for IntoOutputEmitter {
    fn emit_formatted_file(
        &mut self,
        output: &mut dyn Write,
        _printer: &Printer,
        FormattedFile {
            filename,
            formatted_text,
            ..
        }: FormattedFile<'_>,
    ) -> Result<EmitterResult, io::Error> {
        if self.verbosity != Verbosity::Quiet {
            writeln!(output, "{filename}:\n")?;
        }
        write!(output, "{formatted_text}")?;
        Ok(EmitterResult::default())
    }
}
