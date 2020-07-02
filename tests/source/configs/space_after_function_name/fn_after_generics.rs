// rustfmt-space_after_function_name: AfterGenerics
// Function space after function name

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
