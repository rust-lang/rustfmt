use std::panic::{AssertUnwindSafe, catch_unwind};

use rustc_ast::ast;
use rustc_ast::token::TokenKind;
use rustc_parse::exp;
use rustc_parse::parser::attr::InnerAttrPolicy;
use rustc_parse::parser::{AllowConstBlockItems, ForceCollect, Parser};
use rustc_span::symbol::kw;

use crate::parse::macros::build_stream_parser;
use crate::parse::session::ParseSess;

pub(crate) fn parse_cfg_if<'a>(
    psess: &'a ParseSess,
    mac: &'a ast::MacCall,
) -> Result<Vec<ast::Item>, &'static str> {
    let result = catch_unwind(AssertUnwindSafe(|| parse_cfg_if_inner(psess, mac)));
    let dcx = psess.inner().dcx();
    let recovered = dcx.has_errors().is_some();
    dcx.reset_err_count();
    match result {
        Ok(Ok(_)) if recovered => Err("cfg_if! parsed with recovery"),
        Ok(res) => res,
        Err(..) => Err("failed to parse cfg_if!"),
    }
}

fn parse_cfg_if_inner<'a>(
    psess: &'a ParseSess,
    mac: &'a ast::MacCall,
) -> Result<Vec<ast::Item>, &'static str> {
    let ts = mac.args.tokens.clone();
    let mut parser = build_stream_parser(psess.inner(), ts);

    let mut items = vec![];
    let mut process_if_cfg = true;

    while parser.token.kind != TokenKind::Eof {
        if process_if_cfg {
            eat_if(&mut parser)?;
        }

        while parser.token != TokenKind::CloseBrace && parser.token.kind != TokenKind::Eof {
            let item = match parser.parse_item(ForceCollect::No, AllowConstBlockItems::Yes) {
                Ok(Some(item_ptr)) => *item_ptr,
                Ok(None) => {
                    if parser.token.is_keyword(kw::If) {
                        // eat a nested if
                        eat_if(&mut parser)?;
                    } else {
                        // Not sure what token we're on. To prevent infinite loops bump the parser
                        parser.bump();
                    }
                    continue;
                }
                Err(err) => {
                    err.cancel();
                    parser.psess.dcx().reset_err_count();
                    return Err(
                        "Expected item inside cfg_if block, but failed to parse it as an item",
                    );
                }
            };
            if let ast::ItemKind::Mod(..) = item.kind {
                items.push(item);
            }
        }

        if !parser.eat(exp!(CloseBrace)) {
            return Err("Expected a closing brace");
        }

        if parser.token.is_keyword(kw::Else) {
            // there might be an `else` after the `if`
            let _ = parser.eat_keyword(exp!(Else));
            // there might be an opening brace after the `else`, but it might also be an `else if`
            let _ = parser.eat(exp!(OpenBrace));
        }

        if parser.eat(exp!(Eof)) {
            break;
        }

        process_if_cfg = parser.token.is_keyword(kw::If);
    }

    Ok(items)
}

fn eat_if(parser: &mut Parser<'_>) -> Result<(), &'static str> {
    if !parser.eat_keyword(exp!(If)) {
        return Err("Expected `if`");
    }

    if !matches!(parser.token.kind, TokenKind::Pound) {
        return Err("Failed to parse attributes");
    }

    // Inner attributes are not actually syntactically permitted here, but we don't care about inner
    // vs outer attributes in this position. Our purpose with this special case parsing of cfg_if
    // macros is to ensure we can correctly resolve imported modules that may have a custom `path`
    // defined.
    //
    // As such, we just need to advance the parser past the attribute and up to to the opening
    // brace. See also https://github.com/rust-lang/rust/pull/79433
    parser
        .parse_attribute(InnerAttrPolicy::Permitted)
        .map_err(|e| {
            e.cancel();
            "Failed to parse attributes"
        })?;

    if !parser.eat(exp!(OpenBrace)) {
        return Err("Expected an opening brace");
    }
    Ok(())
}
