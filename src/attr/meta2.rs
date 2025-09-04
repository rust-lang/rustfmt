use rustc_ast::ptr::P;
use rustc_ast::tokenstream::{TokenStream, TokenTree};
use rustc_ast::{ast, token};
use rustc_parse::exp;
use rustc_parse::parser::Parser;
use rustc_session::parse::ParseSess;
use rustc_span::Span;

use crate::config::lists::SeparatorTactic;
use crate::expr::rewrite_literal;
use crate::overflow;
use crate::rewrite::{Rewrite, RewriteContext, RewriteResult};
use crate::shape::Shape;
use crate::spanned::Spanned;
use crate::types::{PathContext, rewrite_path};

fn is_eof(token: &token::Token) -> bool {
    matches!(
        token,
        rustc_ast::token::Token {
            kind: rustc_ast::token::TokenKind::Eof,
            ..
        }
    )
}
fn try_parse<'a, T>(
    parser: &mut Parser<'a>,
    parse: impl FnOnce(&mut Parser<'a>) -> rustc_errors::PResult<'a, Option<T>>,
) -> Option<T> {
    let mut fork = parser.clone();
    match parse(&mut fork) {
        Ok(x) => match parser.psess.dcx().has_errors() {
            Some(_) => {
                parser.psess.dcx().reset_err_count();
                None
            }
            None => match x {
                Some(x) => {
                    *parser = fork;
                    Some(x)
                }
                None => None,
            },
        },
        Err(e) => {
            e.cancel();
            parser.psess.dcx().reset_err_count();
            None
        }
    }
}

#[derive(Debug)]
pub(crate) struct MetaItem2 {
    #[allow(dead_code)] // not used here, but part of the ast copied over from rustc
    pub unsafety: ast::Safety,
    pub path: ast::Path,
    pub kind: MetaItemKind2,
    pub span: Span,
}
impl MetaItem2 {
    pub(crate) fn from_attr(attr: &ast::Attribute, context: &RewriteContext<'_>) -> Option<Self> {
        match &attr.kind {
            ast::AttrKind::Normal(normal) => {
                let _guard = context.enter_macro();
                Self::from_attr_item(&normal.item, context.psess.inner())
            }
            ast::AttrKind::DocComment(..) => None,
        }
    }
    fn from_attr_item(attr: &ast::AttrItem, sess: &ParseSess) -> Option<Self> {
        Some(Self {
            kind: MetaItemKind2::from_attr_args(&attr.args, sess)?,
            unsafety: attr.unsafety,
            path: attr.path.clone(),
            span: attr.span(),
        })
    }
    pub(crate) fn has_name(&self, name: rustc_span::Symbol) -> bool {
        self.path == name
    }
    pub(crate) fn value_str(&self) -> Option<rustc_span::Symbol> {
        if let MetaItemKind2::NameValue(expr) = &self.kind {
            if let ast::Expr {
                kind: ast::ExprKind::Lit(token_lit),
                ..
            } = &**expr
            {
                return ast::LitKind::from_token_lit(*token_lit)
                    .ok()
                    .and_then(|it| it.str());
            }
        }
        None
    }
}

#[derive(Debug)]
pub(crate) enum MetaItemKind2 {
    Word,
    List(Vec<MetaItemInner2>),
    NameValue(P<ast::Expr>),
}
impl MetaItemKind2 {
    fn from_attr_args(args: &ast::AttrArgs, sess: &ParseSess) -> Option<Self> {
        match args {
            ast::AttrArgs::Empty => Some(Self::Word),
            ast::AttrArgs::Delimited(ast::DelimArgs {
                dspan: _,
                delim: token::Delimiter::Parenthesis,
                tokens,
            }) => Self::list_from_tokens(tokens.clone(), sess).map(Self::List),
            ast::AttrArgs::Delimited(..) => None,
            ast::AttrArgs::Eq { expr, .. } => Some(Self::NameValue(expr.clone())),
        }
    }

    fn list_from_tokens(tokens: TokenStream, sess: &ParseSess) -> Option<Vec<MetaItemInner2>> {
        let mut parser = Parser::new(sess, tokens, None);
        let mut result = Vec::new();
        while !is_eof(&parser.token) {
            let eat_opt_comma =
                |parser: &mut Parser<'_>| parser.eat(exp!(Eof)) || parser.eat(exp!(Comma));
            let inner = try_parse(&mut parser, |parser| {
                Ok(MetaItemInner2::parse_noexpr(parser).take_if(|_| eat_opt_comma(parser)))
            })
            .or_else(|| {
                try_parse(&mut parser, |parser| {
                    let expr = MetaItemInner2::Expr(parser.parse_expr()?);
                    Ok(eat_opt_comma(parser).then_some(expr))
                })
            });
            result.push(inner?);
        }
        Some(result)
    }
}

#[derive(Debug)]
pub(crate) enum MetaItemInner2 {
    MetaItem(MetaItem2),
    Lit(ast::MetaItemLit),
    Expr(P<ast::Expr>),
}

impl MetaItemInner2 {
    fn parse_noexpr(parser: &mut Parser<'_>) -> Option<Self> {
        if let Some(lit) = ast::MetaItemLit::from_token(&parser.token) {
            parser.bump();
            Some(Self::Lit(lit))
        } else if let token::TokenKind::OpenDelim(token::Delimiter::Invisible(_)) =
            &parser.token.kind
        {
            if let TokenTree::Delimited(.., token::Delimiter::Invisible(_), inner) =
                parser.parse_token_tree()
            {
                Self::parse_noexpr(&mut Parser::new(parser.psess, inner, None))
            } else {
                None
            }
        } else {
            try_parse(parser, |fork| {
                let item = fork.parse_attr_item(rustc_parse::parser::ForceCollect::No)?;
                Ok(MetaItem2::from_attr_item(&item, parser.psess).map(Self::MetaItem))
            })
        }
    }
}

impl Rewrite for MetaItem2 {
    fn rewrite(&self, context: &RewriteContext<'_>, shape: Shape) -> Option<String> {
        self.rewrite_result(context, shape).ok()
    }

    fn rewrite_result(&self, context: &RewriteContext<'_>, shape: Shape) -> RewriteResult {
        Ok(match self.kind {
            MetaItemKind2::Word => {
                rewrite_path(context, PathContext::Type, &None, &self.path, shape)?
            }
            MetaItemKind2::List(ref list) => {
                let path = rewrite_path(context, PathContext::Type, &None, &self.path, shape)?;
                let has_trailing_comma = crate::expr::span_ends_with_comma(context, self.span);
                overflow::rewrite_with_parens(
                    context,
                    &path,
                    list.iter(),
                    // 1 = "]"
                    shape.sub_width(1, self.span)?,
                    self.span,
                    context.config.attr_fn_like_width(),
                    Some(if has_trailing_comma {
                        SeparatorTactic::Always
                    } else {
                        SeparatorTactic::Never
                    }),
                )?
            }
            MetaItemKind2::NameValue(ref expr) => {
                let path = rewrite_path(context, PathContext::Type, &None, &self.path, shape)?;
                // 3 = ` = `
                let lit_shape = shape.shrink_left(path.len() + 3, self.span)?;
                let value = match expr.kind {
                    ast::ExprKind::Lit(ref lit) => {
                        // `rewrite_literal` returns `None` when `lit` exceeds max
                        // width. Since a literal is basically unformattable unless it
                        // is a string literal (and only if `format_strings` is set),
                        // we might be better off ignoring the fact that the attribute
                        // is longer than the max width and continue on formatting.
                        // See #2479 for example.
                        rewrite_literal(context, *lit, expr.span, lit_shape)
                            .unwrap_or_else(|_| context.snippet(expr.span).to_owned())
                    }
                    _ => expr.rewrite_result(context, lit_shape)?,
                };
                format!("{path} = {value}")
            }
        })
    }
}

impl Spanned for MetaItemInner2 {
    fn span(&self) -> Span {
        match self {
            Self::MetaItem(meta) => meta.span,
            Self::Lit(lit) => lit.span,
            Self::Expr(expr) => expr.span(),
        }
    }
}

impl Rewrite for MetaItemInner2 {
    fn rewrite(&self, context: &RewriteContext<'_>, shape: Shape) -> Option<String> {
        self.rewrite_result(context, shape).ok()
    }

    fn rewrite_result(&self, context: &RewriteContext<'_>, shape: Shape) -> RewriteResult {
        match self {
            Self::MetaItem(ref meta_item) => meta_item.rewrite_result(context, shape),
            Self::Lit(ref l) => rewrite_literal(context, l.as_token_lit(), l.span, shape),
            Self::Expr(ref expr) => expr.rewrite_result(context, shape),
        }
    }
}
