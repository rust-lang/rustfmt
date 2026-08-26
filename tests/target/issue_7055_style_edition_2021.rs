// rustfmt-style_edition: 2021
// rustfmt-max_width: 100
// rustfmt-error_on_line_overflow: false

struct S;

impl S {
    const fn new(_: &str) -> Self {
        S
    }

    fn go(&self) {}
}

// The reported symptom. As the receiver of a method call the block was rewritten at
// the full width, the `const ` prefix pushed the result past `max_width`, the chain
// rejected it, and the whole expression was silently left unformatted.
fn receiver_74() {
    const { S::new("aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa") }
        .go();
}

fn receiver_77() {
    const {
        S::new(
            "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa",
        )
    }
    .go();
}

fn receiver_80() {
    const {
        S::new(
            "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa",
        )
    }
    .go();
}

// Not a method receiver, so the over-wide rewrite was accepted and emitted. This is
// the stable formatting the gate protects: on style editions below 2027 these keep
// producing a line past `max_width`.
fn statement_74() {
    const { S::new("aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa") };
}

fn statement_77() {
    const { S::new("aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa") };
}

fn statement_80() {
    const { S::new("aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa") };
}

// The `const ` budget applies to a nested block too.
fn nested() {
    let _ = const {
        const { S::new("aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa") }
    };
}

// Inner attributes belong to the `ConstBlock` node, not the `Block`, so they take the
// `rewrite_block` path directly. Guard that the shape change leaves them alone.
fn inner_attrs() {
    let _ = const {
        #![allow(unused)]
        S::new("aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa")
    };
}
