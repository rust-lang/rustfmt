//@ rustfmt-style_edition: 2027
fn a() {
    let v1 = 1;

    let v2 = 2;
}

fn b() {
    let v1 = 1;

    let v2 = 2;
}

fn c() {
    let v1 = 1;

    // comment
}

fn d() {
    let v1 = 1;

    // comment
    let v2 = 2;
}

fn e() {
    let v1 = 1;

    let v2 = 2;

    // comment
    let v3 = 3;
}

fn f() {
    let v1 = 1;

    {
        // comment
    }
}

fn g() {
    let v1 = 1;

    /* comment */
    let v2 = 2;
}

fn h() {
    let v1 = 1;
    /* comment */
    let v2 = 2;
}

fn i() {
    let v1 = 1;

    /* multi
     * line */
    let v2 = 2;
}

fn j() {
    let v1 = 1;

    // prefix comment
    let v2 = 2;
}
