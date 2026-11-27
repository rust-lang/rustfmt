fn short_generic_args_list_long_method_args_list() {
    x.f::<
        A,
        B, C
        >(AAAAAAAA, BBBBBBBBB, CCCCCCCCC, DDDDDDDDDD, EEEEEEEEEE);
}

fn short_generic_args_list_long_method_args_list_with_inline_comments() {
    x.f::<
        A, // Something about arg A
        B, C
        >(AAAAAAAA, BBBBBBBBB, CCCCCCCCC, DDDDDDDDDD, EEEEEEEEEE);
}

fn short_generic_args_list_long_method_args_list_with_inline_comments_in_both_lists() {
    x.f::<
        A, // Something about arg A
        B, C
        >(AAAAAAAA, BBBBBBBBB, // Something about arg BBBBBBBBB
            CCCCCCCCC, DDDDDDDDDD, EEEEEEEEEE);
}

fn short_generic_args_list_long_method_args_list_with_block_comments() {
    x.f::<
        A, /* Something about arg A */
        B, C
        >(AAAAAAAA, BBBBBBBBB, CCCCCCCCC, DDDDDDDDDD, EEEEEEEEEE);
}

fn short_generic_args_list_long_method_args_list_with_block_comments_in_both_lists() {
    x.f::<
        A, /* Something about arg A */
        B, C
        >(AAAAAAAA, BBBBBBBBB, /* Something about arg BBBBBBBBB */ CCCCCCCCC, DDDDDDDDDD, EEEEEEEEEE);
}
