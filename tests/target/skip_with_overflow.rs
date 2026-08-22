// rustfmt-error_on_line_overflow: true
// rustfmt-error_on_unformatted: true
// rustfmt-max_width: 100

// This is a regression test for a bug where when calculating
// the end of a skip range when use a FmtVisitor's 'line_number' which was _assumed_ to be relative
// to the entire file, but when nesting a visitor inside another would be _relative_ to the start of
// that block

fn foo() {
    let _ = || {
        // the bug: we'd mark the region (lo=16, hi=10) as skipped
        // the lo value is correct, but the hi value is the offset of the line after the end of the
        // 'if' block relative to the start of the '|| {' block
        #[rustfmt::skip]
        if true {
            println!(
                "this is a very long string, it goes over max_width. This is just padding to push it over"
            );
        }
    };
}
