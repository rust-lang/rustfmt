pub(crate) use self::checkstyle::*;
pub(crate) use self::diff::*;
pub(crate) use self::files::*;
pub(crate) use self::files_with_backup::*;
pub(crate) use self::modified_lines::*;
pub(crate) use self::stdout::*;
use crate::FileName;
use std::io;
use std::path::Path;

mod checkstyle;
mod diff;
mod files;
mod files_with_backup;
mod modified_lines;
mod stdout;

pub(crate) struct FormattedFile<'a> {
    pub(crate) filename: &'a FileName,
    pub(crate) formatted_text: &'a str,
    pub(crate) original_text: &'a str,
}

pub(crate) trait Emitter<W> {
    fn emit_formatted_file(
        &self,
        output: &mut W,
        formatted_file: FormattedFile<'_>,
    ) -> Result<bool, io::Error>;

    fn emit_header(&self, _output: &mut W) -> Result<(), io::Error> {
        Ok(())
    }

    fn emit_footer(&self, _output: &mut W) -> Result<(), io::Error> {
        Ok(())
    }
}

fn ensure_real_path(filename: &FileName) -> &Path {
    match *filename {
        FileName::Real(ref path) => path,
        _ => panic!("cannot format `{}` and emit to files", filename),
    }
}
