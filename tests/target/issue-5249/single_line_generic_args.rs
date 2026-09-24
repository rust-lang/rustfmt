fn short_generic_args_list() {
    x.f::<A, B, C>();
}

fn short_generic_args_list_with_inline_comments() {
    x.f::<
        A, // Something about arg A
        B,
        C,
    >();
}

fn short_generic_args_list_with_block_comments() {
    x.f::<A /* Something about arg A */, B, C>();
}
