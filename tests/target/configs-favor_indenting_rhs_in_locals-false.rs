// rustfmt-favor_indenting_rhs_in_locals: false
// Option to prefer indenting RHS in locals vs splitting lines

fn main() {
    let a_really_long_variable_name = if lorem_ipsum_dolor.amet() == 0 && foo_bar_baz.quux != "aaa"
    {
        Some(42)
    } else {
        None
    };
}
