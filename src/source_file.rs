use std::fs;
use std::io::{self, Write};

use crate::NewlineStyle;
use crate::config::FileName;
use crate::emitter::{self, Emitter};

#[cfg(test)]
use crate::config::Config;
#[cfg(test)]
use crate::create_emitter;
#[cfg(test)]
use crate::formatting::FileRecord;

// Append a newline to the end of each file.
pub(crate) fn append_newline(s: &mut String) {
    s.push('\n');
}

#[cfg(test)]
pub(crate) fn write_all_files<T>(
    source_file: &[FileRecord],
    out: &mut T,
    config: &Config,
) -> Result<(), io::Error>
where
    T: Write,
{
    let mut emitter = create_emitter(config);

    emitter.emit_header(out)?;
    for (filename, text) in source_file {
        let original_text = match filename {
            FileName::Real(path) => fs::read_to_string(path)?,
            FileName::Stdin => panic!("cannot format `{filename}` and emit to files"),
        };
        write_file(filename, &original_text, text, out, &mut *emitter)?;
    }
    emitter.emit_footer(out)?;

    Ok(())
}

/// Returns the text of `filename` that the formatted output should be compared
/// against.
///
/// `snippet` is the file's source text as stored in the parse session's source
/// map.
pub(crate) fn original_text(
    filename: &FileName,
    snippet: &str,
    newline_style: NewlineStyle,
) -> Result<String, io::Error> {
    // SourceFile's in the SourceMap will always have Unix-style line endings
    // See: https://github.com/rust-lang/rustfmt/issues/3850
    // So when `filename` is FileName::Real we must check the file system to get
    // the original file value, both to detect the newline style for
    // `NewlineStyle::Auto` and to detect newline_style conflicts.
    // For `FileName::Stdin` the original input is only available from the
    // source map.
    match filename {
        FileName::Real(path) => {
            let text = fs::read_to_string(path)?;
            if newline_style != NewlineStyle::Auto {
                return Ok(text);
            }
            // The source map also strips a leading byte order mark. Keep ignoring
            // it here so that `Auto` only starts reporting newline differences.
            match text.strip_prefix('\u{feff}') {
                Some(text) => Ok(text.to_owned()),
                None => Ok(text),
            }
        }
        FileName::Stdin => Ok(snippet.to_owned()),
    }
}

pub(crate) fn write_file<T>(
    filename: &FileName,
    original_text: &str,
    formatted_text: &str,
    out: &mut T,
    emitter: &mut dyn Emitter,
) -> Result<emitter::EmitterResult, io::Error>
where
    T: Write,
{
    let formatted_file = emitter::FormattedFile {
        filename,
        original_text,
        formatted_text,
    };

    emitter.emit_formatted_file(out, formatted_file)
}
