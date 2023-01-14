/// Returns `true` if the given span is a part of generated files.
pub(super) fn is_generated_file(original_snippet: &str, header_size: usize) -> bool {
    original_snippet
        .lines()
        .take(header_size) // looking for marker only in the beginning of the file
        .any(|line| line.contains("@generated"))
}
