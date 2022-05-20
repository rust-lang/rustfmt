use rustc_ast::token::TokenKind;
use rustc_ast::{ast, ptr};
use rustc_builtin_macros::asm::{parse_asm_args, AsmArgs as RawAsmArgs};
use rustc_span::{Span, Symbol};

use crate::rewrite::RewriteContext;

pub(crate) struct AsmArgs<'a> {
    mac: &'a ast::MacCall,
    asm_args: RawAsmArgs,
    options: Vec<ptr::P<ast::Expr>>,
    clobber_abis: Vec<ptr::P<ast::Expr>>,
}

pub(crate) fn parse_asm<'a>(
    context: &RewriteContext<'_>,
    mac: &'a ast::MacCall,
) -> Option<AsmArgs<'a>> {
    let ts = mac.args.inner_tokens();
    let mut parser = super::build_parser(context, ts);
    let asm_args =
        parse_asm_args(&mut parser, context.parse_sess.inner(), mac.span(), false).ok()?;
    let (options, clobber_abis) = parse_asm_options_and_clobbers_as_expr(context, mac, &asm_args)?;
    Some(AsmArgs {
        mac,
        asm_args,
        options,
        clobber_abis,
    })
}

/// Parse asm options and clobber abis as ast::Expr.
fn parse_asm_options_and_clobbers_as_expr(
    context: &RewriteContext<'_>,
    mac: &ast::MacCall,
    asm_args: &RawAsmArgs,
) -> Option<(Vec<ptr::P<ast::Expr>>, Vec<ptr::P<ast::Expr>>)> {
    if asm_args.options_spans.is_empty() && asm_args.clobber_abis.is_empty() {
        return Some((Vec::new(), Vec::new()));
    }

    let tokens = mac.args.inner_tokens();
    let mut parser = super::build_parser(context, tokens);

    let mut options = Vec::with_capacity(asm_args.options_spans.len());
    let mut clobber_abis = Vec::with_capacity(asm_args.clobber_abis.len());

    let option_symbol = Symbol::intern("options");
    let clobber_abi_symbol = Symbol::intern("clobber_abi");

    while parser.token.kind != TokenKind::Eof {
        if parser.token.is_ident_named(option_symbol) {
            // if an option is found, it should be parsable as a fuction call
            options.push(parser.parse_expr().ok()?)
        } else if parser.token.is_ident_named(clobber_abi_symbol) {
            // if a clobber_abi is found, it should be parsable as a fuction call
            clobber_abis.push(parser.parse_expr().ok()?)
        } else {
            parser.bump();
        }
    }
    Some((options, clobber_abis))
}

impl<'a> AsmArgs<'a> {
    pub(crate) fn mac(&self) -> &'a ast::MacCall {
        self.mac
    }

    pub(crate) fn templates(&self) -> &Vec<ptr::P<ast::Expr>> {
        &self.asm_args.templates
    }

    pub(crate) fn operands(&self) -> &Vec<(ast::InlineAsmOperand, Span)> {
        &self.asm_args.operands
    }

    pub(crate) fn clobber_abis(&self) -> &Vec<ptr::P<ast::Expr>> {
        &self.clobber_abis
    }

    pub(crate) fn options(&self) -> &Vec<ptr::P<ast::Expr>> {
        &self.options
    }
}
