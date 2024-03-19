// rustfmt-version: Two
pub fn main() {
    match a {
        #![deny(non_exhaustive_omitted_patterns)]

        // test
        Expr::Array(ExprArray { attrs, .. }) => None,
        _ => None,
    }

    match b {
        #![deny(non_exhaustive_omitted_patterns)]

        Expr::Array(ExprArray { attrs, .. }) => None,
        _ => None,
    }
}
