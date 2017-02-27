// rustfmt-file_lines: [{"file":"tests/source/file-lines-5.rs","range":[9,10]},{"file":"tests/source/file-lines-5.rs","range":[20,25]}]
// rustfmt-error_on_line_overflow: false

impl A {
    fn foo() {
    }
}

impl B {
    fn foo() {}
}

trait C {
    fn foo() {
    }
}

fn main() {
    let y = if cond { val1 } else { val2 }.method_call();
}

fn bar() {
    {
        {
            // comment
        }
    }
}
