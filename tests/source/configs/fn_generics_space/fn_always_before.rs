// rustfmt-fn_generics_space: OnlyBefore
// rustfmt-fn_no_generics_space: true
// Spacing around function generics

fn lorem() {
    // body
}

fn lorem(ipsum: usize) {
    // body
}

fn lorem<T>(ipsum: T)
where
    T: Add + Sub + Mul + Div,
{
    // body
}
