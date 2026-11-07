// rustfmt-error_on_unformatted: true
// rustfmt-error_on_line_overflow: true

// Part of #4968. This line has a width of 100, so it should be fine, but rustfmt panicked.
fn panic_with_tabs() {
    let a = "tab here:	Lorem ipsum dolor sit amet, consetetur sadipscing elitr, sed diam nonu est \
             est, consetetur sadipscing";
}
