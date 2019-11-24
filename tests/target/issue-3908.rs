// rustfmt-use_try_shorthand: true

macro_rules! foo {
    () => {
        bar1()
            // baz1
            .expect("Qux1");
        bar2()?
            // baz2a
            .expect("Qux2a")?
            // baz2b
            .expect("Qux2b")?;
        bar3()?
            // baz3
            .expect("Qux3")?;
    };
}

fn foo() {
    bar4()?
        // baz4
        .expect("Qux4")?;
}
