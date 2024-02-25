use super::*;
use crate::buf_println;
use std::fs;

#[derive(Debug, Default)]
pub(crate) struct FilesEmitter {
    print_misformatted_file_names: bool,
}

impl FilesEmitter {
    pub(crate) fn new(print_misformatted_file_names: bool) -> Self {
        Self {
            print_misformatted_file_names,
        }
    }
}

impl Emitter for FilesEmitter {
    fn emit_formatted_file(
        &mut self,
        _output: &mut dyn Write,
        printer: &Printer,
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
            if self.print_misformatted_file_names {
                buf_println!(printer, "{}", filename.display());
            }
        }
        Ok(EmitterResult::default())
    }
}
