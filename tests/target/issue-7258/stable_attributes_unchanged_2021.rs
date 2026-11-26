//@ rustfmt-style_edition: 2021

// This test confirms bug compatibility with the existing behavior of style edition 2021, which
// wraps long attribute lines one column earlier than it's supposed to.

struct StableAttrs {
    #[my_very_long_attribute_name_(
        aaaaaaaaaaaa,
        bbbbbbbbbbbb,
        cccccccccccc,
        dddddddddddd,
        eeeeeee
    )]
    wraps_at_100: i32,
    #[my_very_long_attribute_name_(aaaaaaaaaaaa, bbbbbbbbbbbb, cccccccccccc, dddddddddddd, eeeeee)]
    stays_at_99: i32,
}
