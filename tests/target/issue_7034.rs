fn foo(a: foo_ty!(&'a (), &'a ));
fn bar(a: foo_ty!(&'a (), &&'a ));
fn baz(a: foo_ty!(&'a (), &&&'a ));
