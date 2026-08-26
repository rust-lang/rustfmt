// rustfmt-style_edition: 2027
// rustfmt-max_width: 30
// rustfmt-error_on_line_overflow: false

// Fewer than six columns of
// budget left: still format
// it, do not bail out and
// emit it verbatim.
fn deep() {
    if a {
        if b {
            if c {
                if d {
                    if e {
                        let q = const {
                            1 + 2
                        };
                    }
                }
            }
        }
    }
}
