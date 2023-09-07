// rustfmt-version: Two
// rustfmt-single_line_let_else_max_width: 100

fn issue5901() {
    #[cfg(target_os = "linux")]
    let Some(x) = foo else {
        return;
    };

    #[cfg(target_os = "linux")]
    // Some comments between attributes and let-else statement
    let Some(x) = foo else {
        return;
    };

    #[cfg(target_os = "linux")]
    #[cfg(target_arch = "x86_64")]
    let Some(x) = foo else {
        return;
    };

    // The else block will be single-lined because attributes and comments before `let`
    // are no longer included when calculating max width
    #[cfg(target_os = "linux")]
    #[cfg(target_arch = "x86_64")]
    // Some comments between attributes and let-else statement
    let Some(x) = foo else { todo!() };
}
