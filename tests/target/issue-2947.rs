// rustfmt-force_multiline_blocks: true

impl fmt::Display for DeriveError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            DeriveError::UnionUnsupported(name) => {
                write!(f, "Cannot derive `Spaned` for `union {}`", name)
            }
            DeriveError::UnitStructUnsupported(name) => {
                write!(f, "Cannot derive `Spanned` for `struct {};`", name)
            }
            DeriveError::NamedStructLacksSpan(name) => {
                write!(
                    f,
                    "Cannot derive `Spanned` for `struct {}` as it lacks a field `span`",
                    name
                )
            }
            DeriveError::TupleStructNotNewtype(name) => {
                write!(
                    f,
                    "Cannot derive `Spanned` for `struct {}` as it does not have a single tuple member",
                    name,
                )
            }
        }
    }
}
