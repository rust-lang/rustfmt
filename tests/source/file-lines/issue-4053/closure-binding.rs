//@ rustfmt-file_lines: [{"file":"tests/source/file-lines/issue-4053/closure-binding.rs","range":[8,8]}]

// Regression case from https://github.com/rust-lang/rustfmt/issues/5340

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
