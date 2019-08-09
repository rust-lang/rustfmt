use super::*;
use crate::rustfmt_diff::{make_diff, DiffLine, Mismatch};
use serde::Serialize;
use serde_json::to_string as to_json_string;
use std::io::{self, Write};
use std::path::Path;

#[derive(Debug, Default)]
pub(crate) struct JsonEmitter {
    num_files: u32,
}

#[derive(Debug, Default, Serialize)]
struct MismatchedBlock {
    original_begin_line: u32,
    original_end_line: u32,
    expected_begin_line: u32,
    expected_end_line: u32,
    original: String,
    expected: String,
}

#[derive(Debug, Default, Serialize)]
struct MismatchedFile {
    name: String,
    mismatches: Vec<MismatchedBlock>,
}

impl Emitter for JsonEmitter {
    fn emit_header(&self, output: &mut dyn Write) -> Result<(), io::Error> {
        write!(output, "[")?;
        Ok(())
    }

    fn emit_footer(&self, output: &mut dyn Write) -> Result<(), io::Error> {
        write!(output, "]")?;
        Ok(())
    }

    fn emit_formatted_file(
        &mut self,
        output: &mut dyn Write,
        FormattedFile {
            filename,
            original_text,
            formatted_text,
        }: FormattedFile<'_>,
    ) -> Result<EmitterResult, io::Error> {
        const CONTEXT_SIZE: usize = 0;
        let filename = ensure_real_path(filename);
        let diff = make_diff(original_text, formatted_text, CONTEXT_SIZE);
        let has_diff = !diff.is_empty();

        if has_diff {
            output_json_file(output, filename, diff, self.num_files)?;
            self.num_files += 1;
        }

        Ok(EmitterResult { has_diff })
    }
}

fn output_json_file<T>(
    mut writer: T,
    filename: &Path,
    diff: Vec<Mismatch>,
    num_emitted_files: u32,
) -> Result<(), io::Error>
where
    T: Write,
{
    let mut mismatches = vec![];
    for mismatch in diff {
        let original_begin_line = mismatch.line_number_orig;
        let expected_begin_line = mismatch.line_number;
        let mut original_end_line = original_begin_line;
        let mut expected_end_line = expected_begin_line;
        let mut original_line_counter = 0;
        let mut expected_line_counter = 0;
        let mut original_lines = vec![];
        let mut expected_lines = vec![];

        for line in mismatch.lines {
            match line {
                DiffLine::Expected(msg) => {
                    expected_end_line = expected_begin_line + expected_line_counter;
                    expected_line_counter += 1;
                    expected_lines.push(msg)
                }
                DiffLine::Resulting(msg) => {
                    original_end_line = original_begin_line + original_line_counter;
                    original_line_counter += 1;
                    original_lines.push(msg)
                }
                DiffLine::Context(_) => continue,
            }
        }

        mismatches.push(MismatchedBlock {
            original_begin_line,
            original_end_line,
            expected_begin_line,
            expected_end_line,
            original: original_lines.join("\n"),
            expected: expected_lines.join("\n"),
        });
    }
    let json = to_json_string(&MismatchedFile {
        name: String::from(filename.to_str().unwrap()),
        mismatches,
    })?;
    let prefix = if num_emitted_files > 0 { "," } else { "" };
    write!(writer, "{}{}", prefix, &json)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::FileName;
    use std::path::PathBuf;

    #[test]
    fn expected_line_range_correct_when_single_line_split() {
        let file = "foo/bar.rs";
        let mismatched_file = MismatchedFile {
            name: String::from(file),
            mismatches: vec![MismatchedBlock {
                original_begin_line: 79,
                original_end_line: 79,
                expected_begin_line: 79,
                expected_end_line: 82,
                original: String::from("fn Foo<T>() where T: Bar {"),
                expected: String::from("fn Foo<T>()\nwhere\n    T: Bar,\n{"),
            }],
        };
        let mismatch = Mismatch {
            line_number: 79,
            line_number_orig: 79,
            lines: vec![
                DiffLine::Resulting(String::from("fn Foo<T>() where T: Bar {")),
                DiffLine::Expected(String::from("fn Foo<T>()")),
                DiffLine::Expected(String::from("where")),
                DiffLine::Expected(String::from("    T: Bar,")),
                DiffLine::Expected(String::from("{")),
            ],
        };

        let mut writer = Vec::new();
        let exp_json = to_json_string(&mismatched_file).unwrap();
        let _ = output_json_file(&mut writer, &PathBuf::from(file), vec![mismatch], 0);
        assert_eq!(&writer[..], format!("{}", exp_json).as_bytes());
    }
}
