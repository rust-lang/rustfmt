// rustfmt-try_op_density: Wide
// Try operator density

fn foo() {
    bar() ?;
    bar() ?.baz() ?;
    foo() ?
        .foo() ?
        .foo() ?
        .foo() ?
        .foo() ?
        .foo() ?
        .foo() ?
        .foo() ?;
    foo() ?
        .foo() ?
        .foo() ?
        .foo() ?
        .foo() ?
        .foo() ?
        .foo() ?
        .foo() ?
        .foo() ?
        .foo() ?
        .foo() ?
        .foo() ?
        .foo() ?
        .foo() ?
        .foo() ?
        .foo() ?
        .foo() ?
        .foo() ?
        .foo() ?
        .foo() ?
        .foo() ?
        .foo() ?
        .foo() ?
        .foo() ?
        .foo() ?;
}
