use super::*;
use crate::config::Config;
use crate::rustfmt_diff::{make_diff, print_diff};

pub(crate) struct DiffEmitter<'a> {
    config: &'a Config,
}

impl<'a> DiffEmitter<'a> {
    pub(crate) fn new(config: &'a Config) -> Self {
        Self { config }
    }
}

impl<'a> Emitter for DiffEmitter<'a> {
    fn write_file(
        &mut self,
        FormattedFile {
            formatted_text,
            original_text,
            filename,
            ..
        }: FormattedFile<'_>,
    ) -> Result<bool, io::Error> {
        let mismatch = make_diff(&original_text, formatted_text, 3);
        let has_diff = !mismatch.is_empty();
        print_diff(
            mismatch,
            |line_num| format!("Diff in {} at line {}:", filename, line_num),
            self.config,
        );
        return Ok(has_diff);
    }
}
