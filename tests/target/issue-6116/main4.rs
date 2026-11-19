fn foo() -> fn(i32) -> i32 {
    |a| {
        /*comment before empty statement */
        a
    }
}
