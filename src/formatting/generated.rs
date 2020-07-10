use std::{
    fs,
    io::{self, BufRead},
};

use crate::config::file_lines::FileName;
use crate::formatting::comment::contains_comment;

/// Returns `true` if the given span is a part of generated files.
pub(super) fn is_generated_file(file_name: &FileName, original_snippet: Option<&String>) -> bool {
    let first_line = match file_name {
        FileName::Stdin => original_snippet
            .and_then(|s| s.lines().next())
            .map(str::to_owned)
            .unwrap_or("".to_owned()),
        FileName::Real(ref path) => fs::File::open(path)
            .ok()
            .and_then(|f| io::BufReader::new(f).lines().next()?.ok())
            .unwrap_or("".to_owned()),
    };

    is_comment_with_generated_notation(&first_line)
}

fn is_comment_with_generated_notation(s: &str) -> bool {
    contains_comment(&s) && s.contains("@generated")
}

#[cfg(test)]
mod test {
    #[test]
    fn is_comment_with_generated_notation() {
        use super::is_comment_with_generated_notation;

        assert!(is_comment_with_generated_notation("// @generated"));
        assert!(is_comment_with_generated_notation("//@generated\n\n"));
        assert!(is_comment_with_generated_notation("\n// @generated"));
        assert!(is_comment_with_generated_notation("/* @generated"));
    }
}
