use super::*;
use std::fs;

pub(crate) struct FilesEmitter;

impl FilesEmitter {
    pub(crate) fn new() -> Self {
        Self
    }
}

impl<W> Emitter<W> for FilesEmitter {
    fn emit_formatted_file(
        &mut self,
        _output: &mut W,
        FormattedFile {
            original_text,
            formatted_text,
            filename,
            ..
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
