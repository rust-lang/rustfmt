// rustfmt-version: Two

// Original from #4649
trait Foo {
fn bar(&self)
where
//     Self: Bar
// Some comment
;
}

fn foo<T>()
where
//     T: Bar,
// Some comment
{
println!("foo");
}
