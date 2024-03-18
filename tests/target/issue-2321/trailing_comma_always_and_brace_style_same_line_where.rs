// rustfmt-brace_style: SameLineWhere
// rustfmt-trailing_comma: Always

fn lorem<S, T,>(lorem: S, ipsum: T,)
where
    S: Add + Sub,
    T: Mul + Div,
{
    // body
}
