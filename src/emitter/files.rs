use super::*;
use std::fs;

pub(crate) struct FilesEmitter;

impl FilesEmitter {
    pub(crate) fn new() -> Self {
        Self
    }
}

impl Emitter for FilesEmitter {
    fn emit_formatted_file(
        &self,
        _output: &mut dyn Write,
        FormattedFile {
            filename,
            original_text,
            formatted_text,
        }: FormattedFile<'_>,
    ) -> Result<bool, io::Error> {
        // Write text directly over original file if there is a diff.
        let filename = ensure_real_path(filename);
        if original_text != formatted_text {
            fs::write(filename, formatted_text)?;
        }
        Ok(false)
    }
}
