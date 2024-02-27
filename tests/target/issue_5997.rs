// rustfmt-version: Two

// This formats with two spaces:
pub struct Newtype(
    /// Doc
    #[doc()] //
    pub Vec<u8>,
);

// This formats with one:
pub struct Newtype(
    /// Doc
    #[doc()]
    pub Vec<u8>,
);
