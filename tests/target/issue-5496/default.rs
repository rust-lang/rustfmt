fn foo() -> usize {
    let some_long_name = true;
    let some_other_long_name = false;
    let bar = if some_long_name && some_other_long_name {
        baz()
    } else {
        buzz()
    };
    if some_long_name && some_other_long_name {
        1
    } else {
        2
    }
}
