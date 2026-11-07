fn long_generic_args_list() {
    x.f::<
        A,
        A,
        A,
        A,
        A,
        A,
        A,
        A,
        A,
        A,
        A,
        A,
        A,
        A,
        A,
        A,
        A,
        A,
        A,
        A,
        A,
        A,
        A,
        A,
        A,
        A,
        A,
        A,
        A,
        A,
    >();
}

fn long_items_in_generic_args_list() {
    x.f::<
        AAAA<BBBB::CCCC, (DDDD, EEEE), FFFF>,
        AAAA<BBBB::CCCC, (DDDD, EEEE), FFFF>,
        AAAA<BBBB::CCCC, (DDDD, EEEE), FFFF>,
    >();
}
