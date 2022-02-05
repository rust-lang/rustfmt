// #5215
struct MyTuple(
    /// Doc Comments
    /* TODO note to add more to Doc Comments */ u32,
    /// Doc Comments
    // TODO note
    u64,
);

struct MyTuple(
    #[cfg(unix)] // some comment
    u64,
    #[cfg(not(unix))] /*block comment */
    u32,
);

struct MyTuple(
    #[cfg(unix)]
    // some comment
    u64,
    #[cfg(not(unix))]
    /*block comment */
    u32,
);
