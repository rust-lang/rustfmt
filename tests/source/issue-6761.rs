#![feature(associated_type_defaults)]

// Case 1: exact issue repro (comment inside bounds + before-where clause).
pub trait Trait {
    type I: Iterator<
        // This is an item
        Item = Self,
    >
    where
        Self: Copy;
}

// Case 2: comment between the bound and `where`.
trait Bar1 {
    type B: Iterator<Item = u8> // trailing
    where
        Self: Copy;
}

// Case 3: a `/` appears in the bounds without being a comment.
trait Foo<const N: usize> {}
trait Baz {
    type C: Foo<{ 6 / 2 }>
    where
        Self: Copy;
}

// Case 4: bound + RHS + trailing where clause (associated type default).
trait Bound {}
impl Bound for () {}
trait Qux {
    type D: Bound = () where Self: Copy;
}

// Case 5a: associated type in an impl block, no bounds (bounds have no
// effect on impl assoc types and are rejected by rustc), still exercises
// the shared rewrite_ty/where-clause path.
trait Gat {
    type I<T>
    where
        T: Copy;
}
struct S;
impl Gat for S {
    type I<T>
    // comment before where
    where
        T: Copy,
    = Vec<T>;
}

// Case 5b: free type alias, no bounds (bounds have no effect outside trait
// definitions), still exercises the shared rewrite_ty/where-clause path.
type E<T>
// a free comment
where
    T: Copy,
= Vec<T>;

// Case 6: no bounds + comment before `where` (guards the preserved
// generics.span.hi() default).
trait NoBounds {
    type F // just a comment
    where
        Self: Copy;
}

// Case 7: comments around the `:` must not be dropped. The bounds rewrite
// covers only the bounds themselves, so these are recovered separately.
trait AroundColon {
    type G: /* after colon */ Bound
    where
        Self: Copy;

    type H: /* c1 */ Bound + Bound
    where
        Self: Copy;

    type I: // line comment after colon
        Bound
    where
        Self: Copy;

    type J /* before colon */: Bound
    where
        Self: Copy;
}

// Case 8: comment around the `:` on a generic associated type.
trait AroundColonGat {
    type K<U>: /* generics too */ Bound
    where
        U: Copy;
}
