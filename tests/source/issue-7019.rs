fn two() -> usize {
    /*A*/ /*B*/
    1 + 2
}

fn three() -> usize {
    /*A*/ /*B*/ /*C*/
    1 + 2
}

fn nested() -> usize {
    {
        {
            /*A*/ /*B*/ /*C*/
            1 + 2
        }
    }
}

fn between_statements() -> usize {
    let a = 1;
    /*A*/ /*B*/
    let b = 2;
    a + b
}

fn trailing_inline() -> usize {
    let a = 1; /*trailing*/
    a
}

fn already_canonical() -> usize {
    /*A*/
    /*B*/
    1 + 2
}
