fn long_items_in_generic_args_list_inline_comment() {
    x.f::<
        // Pre comment
        AAAA<BBBB::CCCC, (DDDD, EEEE), FFFF>, // Inline
        AAAA<BBBB::CCCC, (DDDD, EEEE), FFFF>,
        AAAA<BBBB::CCCC, (DDDD, EEEE), FFFF>,
        // Post Comment
    >();
}

fn long_items_in_generic_args_list_multi_line_inline_comment() {
    x.f::<
        // Pre comment
        // Pre comment
        // Pre comment
        AAAA<BBBB::CCCC, (DDDD, EEEE), FFFF>, // Inline
        // Inline
        // Inline
        AAAA<BBBB::CCCC, (DDDD, EEEE), FFFF>,
        AAAA<BBBB::CCCC, (DDDD, EEEE), FFFF>,
        // Post Comment
        // Post Comment
        // Post Comment
    >();
}

fn long_items_in_generic_args_list_block_comment() {
    x.f::<
        /* Pre comment */
        AAAA<BBBB::CCCC, (DDDD, EEEE), FFFF>, /* Inline */
        AAAA<BBBB::CCCC, (DDDD, EEEE), FFFF>,
        AAAA<BBBB::CCCC, (DDDD, EEEE), FFFF>,
        /* Post Comment */
    >();
}

fn long_items_in_generic_args_list_multi_line_block_comment() {
    x.f::<
        /* Pre comment
         * Pre comment
         * Pre comment
         */
        AAAA<BBBB::CCCC, (DDDD, EEEE), FFFF>, /* Inline
                                               * Inline
                                               * Inline
                                               */
        AAAA<BBBB::CCCC, (DDDD, EEEE), FFFF>,
        AAAA<BBBB::CCCC, (DDDD, EEEE), FFFF>,
        /* Post Comment
         * Post Comment
         * Post Comment
         */
    >();
}
