//@ rustfmt-struct_field_align_threshold: 100

fn separate_lines() {
    let _ = Self {
        a: 1,
        #[cfg(feature = "some-long-feature-name")]
        longer: 2,
    };
}

fn inline_input() {
    let _ = Self {
        a: 1,
        #[cfg(feature = "test")] longer: 2,
    };
}
