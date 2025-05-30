// max_width = 120
// error_on_line_overflow = true
// style_edition = "2027"

impl EarlyLintPass for NeedlessContinue {
    fn check_expr(&mut self, cx: &EarlyContext<'_>, expr: &Expr) {
        if let ExprKind::Loop(body, label, ..)
        | ExprKind::While(_, body, label)
        | ExprKind::ForLoop { body, label, .. } = &expr.kind
            && !in_external_macro(cx.sess, expr.span)
        {
            check_final_block_stmt(cx, body, label, expr.span.ctxt());
        }
    }
}
