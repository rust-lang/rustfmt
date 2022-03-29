pub enum LongerName {
    One,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
}

pub enum Wrap {
    A(LongerName),
}

pub fn fmt_me(wrap: Wrap) {
    match wrap {
        Wrap::A(
            LongerName::One
            | LongerName::Two
            | LongerName::Three
            | LongerName::Four
            | LongerName::Five
            | LongerName::Six
            | LongerName::Seven
            | LongerName::Eight
            | LongerName::Nine,
        ) => {}
    }
}

pub fn fmt_me_too(wrap: Wrap) {
    match wrap {
        Wrap::A(LongerName::One)
        | Wrap::A(LongerName::Two)
        | Wrap::A(LongerName::Three)
        | Wrap::A(LongerName::Four)
        | Wrap::A(LongerName::Five)
        | Wrap::A(LongerName::Six)
        | Wrap::A(LongerName::Seven)
        | Wrap::A(LongerName::Eight)
        | Wrap::A(LongerName::Nine) => {}
    }
}
