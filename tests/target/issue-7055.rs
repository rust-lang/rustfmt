// rustfmt-style_edition: 2027

struct S;

impl S {
    const fn new(_: &str) -> Self {
        S
    }

    fn go(&self) {}
}

fn main() {
    const {
        S::new("aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa")
    }
    .go();
}

fn second_issue() {
    const {
        S::new("aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa")
    }
    .go();
}
