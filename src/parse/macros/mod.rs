use rustc_ast::ast;
use rustc_ast::token::{Delimiter, NonterminalKind, NtExprKind::*, NtPatKind::*, TokenKind};
use rustc_ast::tokenstream::TokenStream;
use rustc_parse::MACRO_ARGUMENTS;
use rustc_parse::parser::{AllowConstBlockItems, ForceCollect, Parser, Recovery};
use rustc_session::parse::ParseSess;
use rustc_span::symbol;

use crate::macros::MacroArg;
use crate::rewrite::RewriteContext;

pub(crate) mod cfg_if;
pub(crate) mod cfg_select;
pub(crate) mod lazy_static;

fn build_stream_parser<'a>(psess: &'a ParseSess, tokens: TokenStream) -> Parser<'a> {
    Parser::new(psess, tokens, MACRO_ARGUMENTS).recovery(Recovery::Forbidden)
}

fn build_parser<'a>(context: &RewriteContext<'a>, tokens: TokenStream) -> Parser<'a> {
    build_stream_parser(context.psess.inner(), tokens)
}

fn parse_macro_arg<'a, 'b: 'a>(parser: &'a mut Parser<'b>) -> Option<MacroArg> {
    macro_rules! parse_macro_arg {
        ($macro_arg:ident, $nt_kind:expr, $try_parse:expr, $then:expr) => {
            let mut cloned_parser = (*parser).clone();
            if Parser::nonterminal_may_begin_with($nt_kind, &cloned_parser.token)
                || matches!(cloned_parser.token.kind, TokenKind::DocComment(..))
            {
                match $try_parse(&mut cloned_parser) {
                    Ok(x) => {
                        if parser.psess.dcx().has_errors().is_some() {
                            parser.psess.dcx().reset_err_count();
                        } else {
                            // Parsing succeeded.
                            *parser = cloned_parser;
                            return Some(MacroArg::$macro_arg($then(x)?));
                        }
                    }
                    Err(e) => {
                        e.cancel();
                        parser.psess.dcx().reset_err_count();
                    }
                }
            }
        };
    }

    parse_macro_arg!(
        Expr,
        NonterminalKind::Expr(Expr),
        |parser: &mut Parser<'b>| parser.parse_expr(),
        |x: Box<ast::Expr>| Some(x)
    );
    parse_macro_arg!(
        Ty,
        NonterminalKind::Ty,
        |parser: &mut Parser<'b>| parser.parse_ty(),
        |x: Box<ast::Ty>| Some(x)
    );
    parse_macro_arg!(
        Pat,
        NonterminalKind::Pat(PatParam { inferred: false }),
        |parser: &mut Parser<'b>| parser.parse_pat_no_top_alt(None, None),
        |x: ast::Pat| Some(Box::new(x))
    );
    // `parse_item` returns `Option<Box<ast::Item>>`.
    parse_macro_arg!(
        Item,
        NonterminalKind::Item,
        |parser: &mut Parser<'b>| parser.parse_item(ForceCollect::No, AllowConstBlockItems::Yes),
        |x: Option<Box<ast::Item>>| x
    );

    None
}

pub(crate) struct ParsedMacroArgs {
    pub(crate) vec_with_semi: bool,
    pub(crate) trailing_comma: bool,
    pub(crate) args: Vec<MacroArg>,
}

fn check_keyword<'a, 'b: 'a>(parser: &'a mut Parser<'b>) -> Option<MacroArg> {
    if parser.token.is_reserved_ident()
        && parser.look_ahead(1, |t| *t == TokenKind::Eof || *t == TokenKind::Comma)
    {
        let keyword = parser.token.ident().unwrap().0.name;
        parser.bump();
        Some(MacroArg::Keyword(
            symbol::Ident::with_dummy_span(keyword),
            parser.prev_token.span,
        ))
    } else {
        None
    }
}

pub(crate) fn parse_macro_args(
    context: &RewriteContext<'_>,
    tokens: TokenStream,
    style: Delimiter,
    forced_bracket: bool,
) -> Option<ParsedMacroArgs> {
    let mut parser = build_parser(context, tokens);
    let mut args = Vec::new();
    let mut vec_with_semi = false;
    let mut trailing_comma = false;

    if Delimiter::Brace != style {
        loop {
            if let Some(arg) = check_keyword(&mut parser) {
                args.push(arg);
            } else if let Some(arg) = parse_macro_arg(&mut parser) {
                args.push(arg);
            } else {
                return None;
            }

            match parser.token.kind {
                TokenKind::Eof => break,
                TokenKind::Comma => (),
                TokenKind::Semi => {
                    // Try to parse `vec![expr; expr]`
                    if forced_bracket {
                        parser.bump();
                        if parser.token.kind != TokenKind::Eof {
                            match parse_macro_arg(&mut parser) {
                                Some(arg) => {
                                    args.push(arg);
                                    parser.bump();
                                    if parser.token == TokenKind::Eof && args.len() == 2 {
                                        vec_with_semi = true;
                                        break;
                                    }
                                }
                                None => {
                                    return None;
                                }
                            }
                        }
                    }
                    return None;
                }
                _ if args.last().map_or(false, MacroArg::is_item) => continue,
                _ => return None,
            }

            parser.bump();

            if parser.token == TokenKind::Eof {
                trailing_comma = true;
                break;
            }
        }
    }

    Some(ParsedMacroArgs {
        vec_with_semi,
        trailing_comma,
        args,
    })
}

pub(crate) fn parse_expr(
    context: &RewriteContext<'_>,
    tokens: TokenStream,
) -> Option<Box<ast::Expr>> {
    let mut parser = build_parser(context, tokens);
    // FIXME(jieyouxu): double-check error count leak
    parser.parse_expr().ok()
}

/// Synthetic parse failures. See [`try_synthetic_parse`].
enum SyntheticParseFailure {
    Panicked,
    Recovered,
}

/// Attempt to _synthetically_ parse with `synthetic_parse`. This is used for macros where we try to
/// parse as known non-macro Rust syntax to format macro contents that _look_ like valid non-macro
/// Rust syntax.
///
/// * If `synthetic_parse` panics, we will catch the panic and map that to
///   [`SyntheticParseFailure::Panicked`].
/// * If we encounter a parser recovery, we will map that to [`SyntheticParseFailure::Recovered`].
///
/// In either case, we reset shared error count state.
fn try_synthetic_parse<R, P>(
    psess: &crate::parse::session::ParseSess,
    synthetic_parse: P,
) -> Result<R, SyntheticParseFailure>
where
    P: FnOnce() -> R,
{
    let dcx = psess.inner().dcx();

    // Indicative of an error count leak elsewhere.
    debug_assert_eq!(
        dcx.err_count(),
        0,
        "dcx error count should be 0 before calling `try_synthetic_parse`"
    );

    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(synthetic_parse));

    let recovered = dcx.has_errors().is_some();

    // We need to reset error count to prevent shared state from leaking.
    dcx.reset_err_count();

    match result {
        Ok(_) if recovered => Err(SyntheticParseFailure::Recovered),
        Ok(value) => Ok(value),
        Err(..) => Err(SyntheticParseFailure::Panicked),
    }
}
