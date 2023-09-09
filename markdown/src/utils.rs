use unicode_width::UnicodeWidthStr;

// Duplicated from the rustfmt::util module
pub(crate) fn unicode_str_width(s: &str) -> usize {
    s.width()
}
