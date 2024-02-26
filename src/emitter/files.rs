use super::*;
use crate::buf_println;
use std::fs;

#[derive(Default)]
pub(crate) struct FilesEmitter<'a> {
    print_misformatted_file_names: Option<&'a Printer>,
}

impl<'a> FilesEmitter<'a> {
    pub(crate) fn new(print_misformatted_file_names: Option<&'a Printer>) -> Self {
        Self {
            print_misformatted_file_names,
        }
    }
}

impl<'a> Emitter for FilesEmitter<'a> {
    fn emit_formatted_file(
        &mut self,
        _output: &mut dyn Write,
        FormattedFile {
            filename,
            original_text,
            formatted_text,
        }: FormattedFile<'_>,
    ) -> Result<EmitterResult, io::Error> {
        // Write text directly over original file if there is a diff.
        let filename = ensure_real_path(filename);
        if original_text != formatted_text {
            fs::write(filename, formatted_text)?;
            if let Some(printer) = self.print_misformatted_file_names {
                buf_println!(printer, "{}", filename.display());
            }
        }
        Ok(EmitterResult::default())
    }
}
