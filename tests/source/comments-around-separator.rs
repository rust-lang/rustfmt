fn parameters(
    first: bool
    // first line
    ,
    /* second pre */
    second: char /* second */, /* third pre */
    third: u32 /* third, comment */, /* trailing */
) {
}

struct Fields {
    first: i32 /* first */, /* second pre */
    second: i32 /* second */, /* third pre */
    third: i32 /* third, comment */, /* trailing */
}

fn struct_literal() {
    let value = Fields {
        first: 1 /* first */, /* second pre */
        second: 2 /* second */, /* third pre */
        third: 3 /* third, comment */, /* trailing */
    };
}
