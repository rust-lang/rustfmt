// rustfmt-format_code_in_doc_comments: true

/// ```rust
/// impl Test {
///     pub const fn from_bytes(v: &[u8]) -> Result<Self, ParserError> {
///         Self::from_bytes_manual_slice(v, 0, v.len())
///     }
/// }
/// ```

impl Test {
    pub const fn from_bytes(v: &[u8]) -> Result<Self, ParserError> {
        Self::from_bytes_manual_slice(v, 0, v.len())
    }
}
