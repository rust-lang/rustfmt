struct X<'a>(
    #[X(X = "_________________________________________________________________________")]
    pub &'a u32,
    // ^^
);

struct X<'a>(
    #[X(X = "_______________________________________________")]
    pub &'a u32,
    // ^^
);
