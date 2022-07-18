// rustfmt-version: One

// Original from #4649
trait Foo {
    fn bar(&self)
    //     Self: Bar
    // Some comment
    ;
}

fn foo<T>()
//     T: Bar,
// Some comment
{
    println!("foo");
}
