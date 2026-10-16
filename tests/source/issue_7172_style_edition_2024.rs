//@ rustfmt-style_edition: 2024
//@ rustfmt-max_width: 50

#[requires(
x < y &&
x > 0
)]
#[requires({
    let x = x + 1;
    let y = y + 1;
x < y &&
x > 0
}
)]
#[requires(*this_attr_is_49_chars_wide_________)]
#[requires(*this_attr_is_50_chars_wide__________)]
#[requires(*this_attr_is_51_chars_wide___________)]
fn function_with_contract(x: u32, y: u32) {}

fn main() {}
