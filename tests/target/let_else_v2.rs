// rustfmt-version: Two

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
}
