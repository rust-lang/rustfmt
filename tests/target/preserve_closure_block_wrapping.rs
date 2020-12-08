// rustfmt-preserve_closure_block_wrapping: true

fn main() {
    let explicit_conversion_preserves_semantics =
        || { !is_mod || (is_mod && attrs.map_or(true, |a| a.is_empty())) };
}

fn main() {
    || { {} };
}

fn issue1524() {
    let f = |x| {
        {
            {
                { x }
            }
        }
    };
    let f = |x| {
        {
            { x }
        }
    };
    let f = |x| {
        { x }
    };
    let f = |x| { x };
    let f = |x| x;
}

fn main() {
    let arg_test2 =
        |big_argument_name, test123| { looooooooooooooooooong_function_naaaaaaaaaaaaaaaaame() };
}

impl Foo {
    pub fn bar(&self) {
        Some(SomeType {
            push_closure_out_to_100_chars: iter(
                otherwise_it_works_ok.into_iter().map(|f| { Ok(f) }),
            ),
        })
    }
}
