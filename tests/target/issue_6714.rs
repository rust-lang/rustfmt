// rustfmt-style_edition: 2027

// The only difference between the two macros here is that one uses
// `matches!('a', 'a'..='z' | '0'..='9')` while the other use
// `matches!('a', 'a'..='z' | '0')`, without the second range. The problem is
// that the first isn't formated by rustfmt.
// You can test it yourself by using the Rustfmt tool, top right
//
// (Note that in this specific example, I could have used
// `matches!('a', 'a'..='z') || matches!('a', '0'..='9')`, which *is* formated,
// but that's not the point)

macro_rules! this_macro_will_not_be_formated {
    () => {
        let _ = "some ugly not formated code";

        let _ = matches!('a', 'a'..='z' | '0'..='9');
    };
}

macro_rules! this_macro_will_be_formated {
    () => {
        let _ = "some ugly not formated code";

        let _ = matches!('a', 'a'..='z' | '0');
    };
}
