// rustfmt-try_op_density: Compressed
// Try operator density

fn foo() {
    bar()?;
    bar()?.baz()?;
    foo()?.foo()?.foo()?.foo()?.foo()?.foo()?.foo()?.foo()?;
    foo()?
        .foo()?
        .foo()?
        .foo()?
        .foo()?
        .foo()?
        .foo()?
        .foo()?
        .foo()?
        .foo()?
        .foo()?
        .foo()?
        .foo()?
        .foo()?
        .foo()?
        .foo()?
        .foo()?
        .foo()?
        .foo()?
        .foo()?
        .foo()?
        .foo()?
        .foo()?
        .foo()?
        .foo()?;
}
