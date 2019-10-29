// rustfmt-chains_block_parent_indent_children: true
// rustfmt-chains_block_parent_indent_parent_item: Always

fn main() {
    let very_very_very_very_very_very_very_very_very_long_var_name = 13;
    let all = very_very_very_very_very_long_fun_name(
            very_very_very_very_very_very_very_very_very_long_var_name,
        )
        .iter()
        .map(|x| x + very_very_very_very_very_very_long_var_name);
    StructA {
            test_test: some_value,
        }
        .do_stuff(StructB {
            test_test_b: other_value,
        })
        .aaa_aaa()
        .do_stuff(
            StructB {
                    test_test_b: other_value,
                }
                .ddd_ddd()
                .eee_eee(),
        )
        .bbb_bbb()
        .ccc_ccc();
    let more = 13;
    let other = vec![1, 2, 3]
        .iter()
        .map(|x| x + very_very_very_very_very_very_long_var_name);

    foo(|x| {
            // ....
        })
        .bar()
        .baz()
        .unwrap()
}
