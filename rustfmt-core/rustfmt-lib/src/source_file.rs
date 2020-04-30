use std::fs;
use std::io::{self, Write};
use std::path::Path;
use std::rc::Rc;

use crate::config::FileName;
use crate::emitter::{self, Emitter};
use crate::syntux::session::ParseSess;
use crate::{create_emitter, EmitterConfig};
use crate::{FormatReport, FormatResult, NewlineStyle};

// Append a newline to the end of each file.
pub(crate) fn append_newline(s: &mut String) {
    s.push_str("\n");
}

pub fn write_all_files<T>(
    format_report: FormatReport,
    out: &mut T,
    config: EmitterConfig,
) -> Result<bool, io::Error>
where
    T: Write,
{
    let mut emitter = create_emitter(config);
    let mut has_diff = false;

    emitter.emit_header(out)?;
    for (filename, format_result) in format_report.format_result.borrow().iter() {
        has_diff |= write_file(None, filename, &format_result, out, &mut *emitter)?.has_diff;
    }
    emitter.emit_footer(out)?;

    Ok(has_diff)
}

pub(crate) fn write_file<T>(
    parse_sess: Option<&ParseSess>,
    filename: &FileName,
    formatted_result: &FormatResult,
    out: &mut T,
    emitter: &mut dyn Emitter,
) -> Result<emitter::EmitterResult, io::Error>
where
    T: Write,
{
    fn ensure_real_path(filename: &FileName) -> &Path {
        match *filename {
            FileName::Real(ref path) => path,
            _ => panic!("cannot format `{}` and emit to files", filename),
        }
    }

    // SourceFile's in the SourceMap will always have Unix-style line endings
    // See: https://github.com/rust-lang/rustfmt/issues/3850
    // So if the user has explicitly overridden the rustfmt `newline_style`
    // config and `filename` is FileName::Real, then we must check the file system
    // to get the original file value in order to detect newline_style conflicts.
    // Otherwise, parse session is around (cfg(not(test))) and newline_style has been
    // left as the default value, then try getting source from the parse session
    // source map instead of hitting the file system. This also supports getting
    // original text for `FileName::Stdin`.
    let original_text =
        if formatted_result.newline_style != NewlineStyle::Auto && *filename != FileName::Stdin {
            Rc::new(fs::read_to_string(ensure_real_path(filename))?)
        } else {
            match &formatted_result.original_snippet {
                Some(original_snippet) => Rc::new(original_snippet.to_owned()),
                None => match parse_sess.and_then(|sess| sess.get_original_snippet(filename)) {
                    Some(ori) => ori,
                    None => Rc::new(fs::read_to_string(ensure_real_path(filename))?),
                },
            }
        };

    let formatted_file = emitter::FormattedFile {
        filename,
        original_text: original_text.as_str(),
        formatted_text: &formatted_result.formatted_snippet.snippet,
    };

    emitter.emit_formatted_file(out, formatted_file)
}
