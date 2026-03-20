//! headers are sets of consecutive keywords and tokens, such as
//! `pub const unsafe fn foo` and `pub(crate) unsafe trait Bar`.
//!
//! This module contains general logic for formatting such headers,
//! where they are always placed on a single line except when there
//! are comments between parts of the header.

use std::borrow::Cow;

use rustc_ast as ast;
use rustc_span::symbol::Ident;
use rustc_span::{DUMMY_SP, Span};
use tracing::debug;

use crate::comment::{combine_strs_with_missing_comments, contains_comment};
use crate::rewrite::RewriteContext;
use crate::shape::Shape;
use crate::source_map::SpanUtils;
use crate::utils::rewrite_ident;

pub(crate) fn format_header(
    context: &RewriteContext<'_>,
    shape: Shape,
    parts: Vec<HeaderPart<'_>>,
) -> String {
    debug!(?parts, "format_header");
    let shape = shape.infinite_width();

    // Empty `HeaderPart`s are ignored.
    let mut parts = parts.into_iter().filter(|x| !x.snippet.is_empty());
    let Some(part) = parts.next() else {
        return String::new();
    };

    let mut result = part.snippet.into_owned();
    let mut span = part.span;

    for part in parts {
        debug!(?result, "before combine");
        let comments_span = span.between(part.span);
        let comments_snippet = context.snippet(comments_span);
        result = if contains_comment(comments_snippet) {
            // FIXME(fee1-dead): preserve (potentially misaligned) comments instead of reformatting
            // them. Revisit this once we have a strategy for properly dealing with them.
            format!("{result}{comments_snippet}{}", part.snippet)
        } else {
            combine_strs_with_missing_comments(
                context,
                &result,
                &part.snippet,
                comments_span,
                shape,
                true,
            )
            .unwrap_or_else(|_| format!("{} {}", &result, part.snippet))
        };
        debug!(?result);
        span = part.span;
    }

    result
}

#[derive(Debug)]
pub(crate) struct HeaderPart<'a> {
    /// snippet of this part without surrounding space
    snippet: Cow<'a, str>,
    span: Span,
}

impl<'a> HeaderPart<'a> {
    /// Create a new empty HeaderPart with a DUMMY_SP.
    /// Because the snippet is empty these will be skipped when calling `format_header`.
    fn empty() -> Self {
        Self::new("", DUMMY_SP)
    }

    /// Find the `keyword` for the header within the give `span`
    pub(crate) fn keyword(
        context: &RewriteContext<'_>,
        span: Span,
        keyword: impl Into<Cow<'a, str>>,
    ) -> Self {
        let snippet = keyword.into();
        let span = context.snippet_provider.subspan(span, &snippet);
        Self::new(snippet, span)
    }

    pub(crate) fn new(snippet: impl Into<Cow<'a, str>>, span: Span) -> Self {
        Self {
            snippet: snippet.into(),
            span,
        }
    }

    pub(crate) fn ident(context: &'a RewriteContext<'_>, ident: Ident) -> Self {
        Self::new(rewrite_ident(context, ident), ident.span)
    }

    pub(crate) fn visibility(context: &RewriteContext<'_>, vis: &ast::Visibility) -> Self {
        let snippet = match vis.kind {
            ast::VisibilityKind::Public => Cow::from("pub"),
            ast::VisibilityKind::Inherited => Cow::from(""),
            ast::VisibilityKind::Restricted { ref path, .. } => {
                let ast::Path { ref segments, .. } = **path;
                let mut segments_iter =
                    segments.iter().map(|seg| rewrite_ident(context, seg.ident));
                if path.is_global() {
                    segments_iter
                        .next()
                        .expect("Non-global path in pub(restricted)?");
                }
                let is_keyword = |s: &str| s == "crate" || s == "self" || s == "super";
                let path = segments_iter.collect::<Vec<_>>().join("::");
                let in_str = if is_keyword(&path) { "" } else { "in " };

                // FIXME(fee1-dead): comments around parens
                Cow::from(format!("pub({}{})", in_str, path))
            }
        };

        Self::new(snippet, vis.span)
    }

    pub(crate) fn constness(constness: ast::Const) -> Self {
        match constness {
            ast::Const::No => Self::empty(),
            ast::Const::Yes(span) => Self::new("const", span),
        }
    }

    pub(crate) fn safety(safety: ast::Safety) -> Self {
        match safety {
            ast::Safety::Default => Self::empty(),
            ast::Safety::Unsafe(span) => Self::new("unsafe", span),
            ast::Safety::Safe(span) => Self::new("safe", span),
        }
    }

    pub(crate) fn auto(context: &RewriteContext<'_>, span: Span, is_auto: ast::IsAuto) -> Self {
        match is_auto {
            ast::IsAuto::No => Self::empty(),
            ast::IsAuto::Yes => Self::keyword(context, span, "auto"),
        }
    }
}
