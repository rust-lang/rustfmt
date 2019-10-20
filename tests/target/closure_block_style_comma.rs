// rustfmt-indent_style: Block
// rustfmt-version: Two

fn foo() {
    {
        let write_status = |
            status: &mut Vec<ansi_term::ANSIString>,
            diff: &Diff,
            heading: &str,
            color: &Style,
            show_hints: bool,
            hints: &[&str],
        | -> Option<bool> { Some(true) };
    }
}

fn bar() {
    let write_status = |
        status: &mut Vec<ansi_term::ANSIString>,
        diff: &Diff,
        heading: &str,
        color: &Style,
    | -> Option<bool> { Some(true) };
    let baz = |foo: bool| -> Option<bool> { Some(true) };
}
