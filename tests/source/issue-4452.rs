fn a(&self) -> i64 {
    #[allow(clippy::cast_possible_wrap)]
    1u64 as i64
}

fn b(&self) -> i64 {
    #[allow(clippy::cast_possible_wrap)]
    #[attr]
    1..2
}

fn foo() {
    #[allow(clippy::cast_possible_wrap)]
    {}
}
