// rustfmt-format_strings: true

fn x() {
    const ASCII_ESCAPE: &str = "id\u{1f}1\u{1f}/Users/nixon/dev/rs/gitstatusd\u{1f}1c9be4fe5460a30e70de9cbf99c3ec7064296b28\u{1f}master\u{1f}\u{1f}\u{1f}\u{1f}\u{1f}7\u{1f}0\u{1f}1\u{1f}0\u{1f}1\u{1f}0\u{1f}0\u{1f}0\u{1f}\u{1f}0\u{1f}0\u{1f}0\u{1f}\u{1f}\u{1f}0\u{1f}0\u{1f}0\u{1f}0";
    let _ = "\"\"\"\"\"\"\"\"\"\"\"\"\"\"\"\"\"\"\"\"\"\"\"\"\"\"\"\"\"\"\"\"\"\"\"\"\"\"\"\"\"\"\"\"\"";
    let _ = "\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\";
    let _ = "a\x61a\x61a\x61a\x61a\x61a\x61a\x61a\x61a\x61a\x61a\x61a\x61a\x61a\x61a\x61a\x61a\x61";
    let _ = "a\x61aaaa\x61a\x61a\x61aaaa\x61a\x61a\x61a\x61a\x61a\x61a\x61a\x61a\x61a\x61aaaaaa\x61a\x61a\x61a";

    "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa\\a";
    "\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\";
    "\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\";
}