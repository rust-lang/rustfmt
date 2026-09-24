// rustfmt-file_lines: [{"file":"tests/source/issue-4053/closure-binding.rs","range":[11,11]}]
// rustfmt-edition: 2021
// rustfmt-style_edition: 2024
// rustfmt-use_small_heuristics: Max
// rustfmt-newline_style: Unix
// rustfmt-wrap_comments: true

fn GenerateBindingsImpl() {
    catch_unwind(|| {
        // It is ok to abort here.
        let Bindings { rs_api, rs_api_impl } = generate_bindings(json, foobar_support_path, &rustfmt_config_path).unwrap();
        FfiBindings {
            rs_api: FfiU8SliceBox::from_boxed_slice(rs_api.into_bytes().into_boxed_slice()),
            rs_api_impl: FfiU8SliceBox::from_boxed_slice(
                rs_api_impl.into_bytes().into_boxed_slice(),
            ),
        }
    })
}
