fn bar() -> fn(i32) -> i32 {
    |a| {
        a;
        b
    }
}
