// rustfmt-brace_style: SameLineWhere
// rustfmt-trailing_comma: Vertical

fn lorem<S, T>(lorem: S, ipsum: T)
where
    S: Add + Sub,
    T: Mul + Div,
{
    // body
}
