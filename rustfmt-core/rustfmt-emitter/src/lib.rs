pub use self::checkstyle::*;
pub use self::diff::*;
pub use self::files::*;
pub use self::json::*;
pub use self::modified_lines::*;
pub use self::stdout::*;

use std::io::{self, Write};
use std::path::Path;

use rustfmt_configuration::FileName;

pub mod checkstyle;
pub mod diff;
pub mod files;
pub mod json;
pub mod modified_lines;
pub mod rustfmt_diff;
pub mod stdout;

pub struct FormattedFile<'a> {
    pub filename: &'a FileName,
    pub original_text: &'a str,
    pub formatted_text: &'a str,
}

#[derive(Debug, Default, Clone)]
pub struct EmitterResult {
    pub has_diff: bool,
}

pub trait Emitter {
    fn emit_formatted_file(
        &mut self,
        output: &mut dyn Write,
        formatted_file: FormattedFile<'_>,
    ) -> Result<EmitterResult, io::Error>;

    fn emit_header(&self, _output: &mut dyn Write) -> Result<(), io::Error> {
        Ok(())
    }

    fn emit_footer(&self, _output: &mut dyn Write) -> Result<(), io::Error> {
        Ok(())
    }
}

fn ensure_real_path(filename: &FileName) -> &Path {
    match *filename {
        FileName::Real(ref path) => path,
        _ => panic!("cannot format `{}` and emit to files", filename),
    }
}
