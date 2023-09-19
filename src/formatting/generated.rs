/// Returns `true` if the given span is a part of generated files.
pub(super) fn is_generated_file(original_snippet: &str, scan_number_of_lines: usize) -> bool {
    original_snippet
        .lines()
        .take(scan_number_of_lines)
        .any(|line| line.contains("@generated"))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn detect_generated_file_simple() {
        let snippet = "@generated";
        assert!(is_generated_file(snippet, 1));
    }

    #[test]
    fn detect_generated_file_5_lines() {
        let snippet = "\n \n \n \n @generated";
        assert!(is_generated_file(snippet, 5));
    }

    #[test]
    fn no_detect_generated_file_5_lines() {
        let snippet = "\n \n \n \n \n @generated";
        assert!(!is_generated_file(snippet, 5));
    }
}
