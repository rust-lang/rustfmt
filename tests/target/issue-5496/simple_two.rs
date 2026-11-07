// rustfmt-version: Two
fn foo() -> usize {
    // empty
    let _ = if true {
        1
    } else {
    };
    let _ = if true {
    } else {
        2
    };
    let _ = if true {
    } else {
    };

    // attribute
    let _ = if true {
        #[must_use]
        1
    } else {
        2
    };
    let _ = if true {
        1
    } else {
        #[must_use]
        2
    };
    let _ = if true {
        #[must_use]
        1
    } else {
        #[must_use]
        2
    };

    // comment
    let _ = if true {
        1 /*1*/
    } else {
        2
    };
    let _ = if true {
        1
    } else {
        2 /*2*/
    };
    let _ = if true {
        1 /*1*/
    } else {
        2 /*2*/
    };

    // a statement
    let _ = if true {
        let a = 1;
        a
    } else {
        2
    };
    let _ = if true {
        1
    } else {
        let b = 2;
        b
    };
    let _ = if true {
        let a = 1;
        a
    } else {
        let b = 2;
        b
    };

    1
}
