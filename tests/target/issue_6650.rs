// rustfmt-style_edition: 2027

fn main() {
    matches!(
        stmt,
        ast::Stmt {
            kind:
                ast::StmtKind::MacCall(box ast::MacCallStmt {
                    style: ast::MacStmtStyle::Braces,
                    ..
                }),
            ..
        }
    )
}
