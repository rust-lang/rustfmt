// rustfmt-max_width: 100

// line below is 80 chars
pub(super) unsafe trait ALetsMakeThisLineLongerByExactlyFiftyCharacters1234: B {
    fn foo();
}

// line below is 86 chars
pub unsafe trait Trait2LetsMakeThisLineLongerByExactlyFiftyCharacters1234: A + B + C {
    fn foo();
}

// line below is 93 chars
pub trait TraitThreeLetsMakeThisLineLongerByExactlyFiftyCharacters1234: Super1 + Super2 + X {
    fn foo();
}

// line below is 97 chars
trait TraitNumberFourLetsMakeThisLineLongerByExactlyFiftyCharacters1234: Super1 + Super2 + Abcd {
    fn foo();
}

// line below is correctly not wrapped
pub const THIS_LINE_IS_100_CHARS: &str = "12345678901234567890123456789012345678901234567890123456";
