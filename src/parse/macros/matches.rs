use rustc_ast::ast;
use rustc_ast::ptr::P;
use rustc_ast::tokenstream::TokenStream;
use rustc_parse::exp;
use rustc_parse::parser::{CommaRecoveryMode, RecoverColon, RecoverComma};

use super::is_token_tree_comma;
use crate::rewrite::RewriteContext;

#[derive(Debug)]
pub(crate) struct Matches {
    pub(crate) expr: P<ast::Expr>,
    pub(crate) pat: P<ast::Pat>,
    pub(crate) guard: Option<P<ast::Expr>>,
}

/// Parse matches! from <https://doc.rust-lang.org/std/macro.matches.html>
pub(crate) fn parse_matches(context: &RewriteContext<'_>, ts: TokenStream) -> Option<Matches> {
    let mut cursor = ts.iter().peekable();
    // remove trailing commmas from the TokenStream since they lead to errors when parsing ast::Pat
    // using parse_pat_allow_top_alt below since the parser isn't expecting a trailing comma.
    // This is only an issue when the `ast::Pat` is not followed by a guard. In either case it's ok
    // to remove the comma from the stream since we don't need it to parse into a Matches struct
    let mut token_trees = vec![];
    while let Some(tt) = cursor.next() {
        let is_last = cursor.peek().is_none();
        if !(is_last && is_token_tree_comma(tt)) {
            token_trees.push(tt.clone())
        }
    }

    let ts = token_trees.into_iter().collect();
    let mut parser = super::build_parser(context, ts);
    let expr = parser.parse_expr().ok()?;

    let _ = parser.eat(exp!(Comma));

    let pat = parser
        .parse_pat_allow_top_guard(
            None,
            RecoverComma::Yes,
            RecoverColon::Yes,
            CommaRecoveryMode::EitherTupleOrPipe,
        )
        .ok()?;

    let guard = if parser.eat_keyword(exp!(If)) {
        Some(parser.parse_expr().ok()?)
    } else {
        None
    };
    Some(Matches { expr, pat, guard })
}

impl Matches {
    pub(crate) fn items(self) -> [MatchesMacroItem; 2] {
        [
            MatchesMacroItem::Expr(self.expr),
            MatchesMacroItem::Arm(self.pat, self.guard),
        ]
    }
}

#[derive(Debug)]
pub(crate) enum MatchesMacroItem {
    Expr(P<ast::Expr>),
    Arm(P<ast::Pat>, Option<P<ast::Expr>>),
}
