// rustfmt-documented_struct_field_blank_lines: Threshold
// rustfmt-documented_struct_field_blank_lines_threshold: 3
// rustfmt-unstable: true

struct BelowThreshold {
    alpha: u32,
    /// Beta.
    beta: u32,
}

struct AtThreshold {
    alpha: u32,

    /// Beta.
    beta: u32,

    /// Gamma.
    gamma: u32,
}
