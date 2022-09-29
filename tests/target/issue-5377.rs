fn must_end_with_semi() {
    loop {
        break false;
    };
}

fn cannot_end_with_semi() -> bool {
    loop {
        break false;
    }
}
