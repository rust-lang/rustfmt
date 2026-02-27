// rustfmt-max_width: 90
// rustfmt-match_block_trailing_comma: false
// rustfmt-trailing_comma: Never

fn main() {
    match FooBar::Foo {
        FooBar::Foo => unreachable!(),
        FooBar::Bar => {
            println!("match_block_trailing_comma: false");
            println!("trailing_comma: Never");
        }
        FooBar::Baz => {
            println!("Lorem ipsum dolor sit amet, consectetuer adipiscing elit.")
        }
    }
}
