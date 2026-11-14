// rustfmt-max_width: 50

// line below is 30 chars
pub(super) unsafe trait A: B {
    fn foo();
}

// line below is 36 chars
pub unsafe trait Trait2: A + B + C {
    fn foo();
}

// line below is 43 chars
pub trait TraitThree: Super1 + Super2 + X {
    fn foo();
}

// line below is 47 chars
trait TraitNumberFour: Super1 + Super2 + Abcd {
    fn foo();
}

// line below is correctly not wrapped
pub const THIS_LINE_IS_50_CHARS: usize = 12345678;
