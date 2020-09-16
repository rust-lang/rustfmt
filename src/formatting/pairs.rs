use rustc_ast::ast;

use crate::config::lists::*;
use crate::config::IndentStyle;
use crate::formatting::{
    comment::{indent_str_by_last_line_comment, rewrite_comment, rewrite_missing_comment},
    rewrite::{Rewrite, RewriteContext},
    shape::Shape,
    utils::{
        first_line_width, is_single_line, last_line_used_width, last_line_width,
        longest_line_width, mk_sp, trimmed_last_line_width, wrap_str,
    },
};

/// Sigils that decorate a binop pair.
#[derive(Clone, Copy)]
pub(crate) struct PairParts<'a> {
    prefix: &'a str,
    infix: &'a str,
    suffix: &'a str,
}

impl<'a> PairParts<'a> {
    /// Constructs a new `PairParts`.
    pub(crate) fn new(prefix: &'a str, infix: &'a str, suffix: &'a str) -> Self {
        Self {
            prefix,
            infix,
            suffix,
        }
    }

    pub(crate) fn infix(infix: &'a str) -> PairParts<'a> {
        PairParts {
            prefix: "",
            infix,
            suffix: "",
        }
    }
}

// Flattens a tree of pairs into a list and tries to rewrite them all at once.
// FIXME would be nice to reuse the lists API for this, but because each separator
// can be different, we can't.
pub(crate) fn rewrite_all_pairs(
    expr: &ast::Expr,
    shape: Shape,
    context: &RewriteContext<'_>,
) -> Option<String> {
    expr.flatten(context, shape).and_then(|list| {
        // First we try formatting on one line.
        rewrite_pairs_one_line(&list, shape, context)
            .or_else(|| rewrite_pairs_multiline(&list, shape, context))
    })
}

// This may return a multi-line result since we allow the last expression to go
// multiline in a 'single line' formatting.
fn rewrite_pairs_one_line<T: Rewrite>(
    list: &PairList<'_, '_, T>,
    shape: Shape,
    context: &RewriteContext<'_>,
) -> Option<String> {
    assert!(list.list.len() >= 2, "Not a pair?");

    let mut result = String::new();
    let base_shape = shape.block();

    let mut prefix_iter = list.sep_prefixes.iter();
    let mut suffix_iter = list.sep_suffixes.iter();
    for ((_, rewrite), s) in list.list.iter().zip(list.separators.iter()) {
        if let Some(rewrite) = rewrite {
            if !is_single_line(&rewrite) || result.len() > shape.width {
                return None;
            }

            result.push_str(&rewrite);
            result.push(' ');
            let c = prefix_iter.next()?.trim();
            if !c.is_empty() {
                if c.starts_with("//") {
                    return None;
                } else {
                    result.push_str(c);
                    result.push(' ');
                };
            };
            result.push_str(s);
            result.push(' ');
            let c = suffix_iter.next()?.trim();
            if !c.is_empty() {
                if c.starts_with("//") {
                    return None;
                } else {
                    result.push_str(c);
                    result.push(' ');
                };
            };
        } else {
            return None;
        }
    }

    let prefix_len = result.len();
    let last = list.list.last()?.0;
    let cur_shape = base_shape.offset_left(last_line_width(&result))?;
    let last_rewrite = last.rewrite(context, cur_shape)?;
    result.push_str(&last_rewrite);

    if first_line_width(&result) > shape.width {
        return None;
    }

    // Check the last expression in the list. We sometimes let this expression
    // go over multiple lines, but we check for some ugly conditions.
    if !(is_single_line(&result) || last_rewrite.starts_with('{'))
        && (last_rewrite.starts_with('(') || prefix_len > context.config.tab_spaces())
    {
        return None;
    }

    wrap_str(result, context.config.max_width(), shape)
}

fn rewrite_pairs_multiline<T: Rewrite>(
    list: &PairList<'_, '_, T>,
    shape: Shape,
    context: &RewriteContext<'_>,
) -> Option<String> {
    let rhs_offset = shape.rhs_overhead(&context.config);
    let nested_shape = (match context.config.indent_style() {
        IndentStyle::Visual => shape.visual_indent(0),
        IndentStyle::Block => shape.block_indent(context.config.tab_spaces()),
    })
    .with_max_width(&context.config)
    .sub_width(rhs_offset)?;

    let indent_str = nested_shape.indent.to_string_with_newline(context.config);
    let mut result = String::new();

    result.push_str(&list.list[0].1.as_ref()?);

    let mut prefix_iter = list.sep_prefixes.iter();
    let mut suffix_iter = list.sep_suffixes.iter();
    let mut first_iter = true;
    for ((e, default_rw), s) in list.list[1..].iter().zip(list.separators.iter()) {
        // The following test checks if we should keep two subexprs on the same
        // line. We do this if not doing so would create an orphan and there is
        // enough space to do so.

        /* First multiline comment prefix and suffix in SeparatorPlace::Back
         * and that are added to existing line requires less alignment, compared to
         * the other comments that are added to already block aligned lines */
        let multiline_align_overhead = if first_iter {
            first_iter = false;
            0
        } else {
            shape.indent.block_indent
        };
        let prefix = prefix_iter.next()?.trim();
        let suffix = suffix_iter.next()?.trim();
        let prelen = if prefix.is_empty() {
            0
        } else {
            longest_line_width(&prefix) + 1 /* +1 if for separator suffix */
        };
        let suflen = if suffix.is_empty() {
            0
        } else {
            longest_line_width(&suffix) + 1 /* +1 if for separator suffix */
        };
        let offset = if result.contains('\n') {
            0
        } else {
            shape.used_width()
        };
        if last_line_width(&result) + offset <= nested_shape.used_width() {
            // We must snuggle the next line onto the previous line to avoid an orphan.
            if let Some(line_shape) =
                shape.offset_left(s.len() + 2 + trimmed_last_line_width(&result) + prelen + suflen)
            {
                if let Some(rewrite) = e.rewrite(context, line_shape) {
                    result.push(' ');
                    if prelen > 0 {
                        result.push_str(&rewrite_comment(
                            &prefix,
                            true,
                            shape.block_indent(
                                last_line_used_width(&result, shape.offset)
                                    - multiline_align_overhead,
                            ),
                            context.config,
                        )?);
                        result.push_str(indent_str_by_last_line_comment(&prefix, &indent_str, " "));
                    };
                    result.push_str(s);
                    result.push(' ');
                    if suflen > 0 {
                        result.push_str(&rewrite_comment(
                            &suffix,
                            true,
                            shape.block_indent(
                                last_line_used_width(&result, shape.offset)
                                    - multiline_align_overhead,
                            ),
                            context.config,
                        )?);
                        result.push_str(indent_str_by_last_line_comment(&suffix, &indent_str, " "));
                    };
                    result.push_str(&rewrite);
                    continue;
                }
            }
        }

        /* Add pre-separator comment  - try to add at the end of current last line */
        if prelen > 0 {
            let align = if trimmed_last_line_width(&result) + prelen > shape.width {
                result.push_str(&indent_str);
                shape.indent.block_indent
            } else {
                result.push(' ');
                multiline_align_overhead
            };
            result.push_str(&rewrite_comment(
                &prefix,
                true,
                shape.block_indent(last_line_used_width(&result, shape.offset) - align),
                context.config,
            )?);
        }
        /* Add separator - in multiline each separator starts or ends a line */
        match context.config.binop_separator() {
            SeparatorPlace::Back => {
                let mut sep_in_new_line = false; /* whether separator starts a new line */
                /* add separator */
                if trimmed_last_line_width(&result) + s.len() + 1 > shape.width {
                    result.push_str(&indent_str);
                    sep_in_new_line = true;
                } else {
                    result.push(' ');
                }
                result.push_str(s);
                /* Add post-separator comment - same line with seperator if possible */
                if suflen > 0 {
                    let align = if trimmed_last_line_width(&result) + suflen > shape.width
                        && !sep_in_new_line
                    {
                        result.push_str(&indent_str);
                        shape.indent.block_indent
                    } else {
                        result.push(' ');
                        multiline_align_overhead
                    };
                    result.push_str(&rewrite_comment(
                        &suffix,
                        true,
                        shape.block_indent(last_line_used_width(&result, shape.offset) - align),
                        context.config,
                    )?);
                };
                result.push_str(&indent_str);
            }
            SeparatorPlace::Front => {
                result.push_str(&indent_str);
                result.push_str(s);
                /* Add post-separator comment at the same line with seperator
                 *  so it will not be left alone in the line */
                if suflen > 0 {
                    result.push(' ');
                    result.push_str(&rewrite_comment(
                        &suffix,
                        true,
                        shape.block_indent(
                            last_line_used_width(&result, shape.offset) - shape.indent.block_indent,
                        ),
                        context.config,
                    )?);
                }
                /* Whether rhs is in the same or new line */
                let l = match default_rw {
                    Some(x) => {
                        if suflen > 0 {
                            first_line_width(&x)
                        } else {
                            0
                        }
                    }
                    None => 0,
                };
                if trimmed_last_line_width(&result) + 1 + l > shape.width {
                    result.push_str(&indent_str);
                } else {
                    result.push(' ');
                }
            }
        }

        result.push_str(&default_rw.as_ref()?);
    }
    Some(result)
}

// Rewrites a single pair.
pub(crate) fn rewrite_pair<LHS, RHS>(
    lhs: &LHS,
    rhs: &RHS,
    pp: PairParts<'_>,
    context: &RewriteContext<'_>,
    shape: Shape,
    separator_place: SeparatorPlace,
) -> Option<String>
where
    LHS: Rewrite,
    RHS: Rewrite,
{
    let tab_spaces = context.config.tab_spaces();
    let lhs_overhead = match separator_place {
        SeparatorPlace::Back => shape.used_width() + pp.prefix.len() + pp.infix.trim_end().len(),
        SeparatorPlace::Front => shape.used_width(),
    };
    let lhs_shape = Shape {
        width: context.budget(lhs_overhead),
        ..shape
    };
    let lhs_result = lhs
        .rewrite(context, lhs_shape)
        .map(|lhs_str| format!("{}{}", pp.prefix, lhs_str))?;

    // Try to put both lhs and rhs on the same line.
    let rhs_orig_result = shape
        .offset_left(last_line_width(&lhs_result) + pp.infix.len())
        .and_then(|s| s.sub_width(pp.suffix.len()))
        .and_then(|rhs_shape| rhs.rewrite(context, rhs_shape));
    if let Some(ref rhs_result) = rhs_orig_result {
        // If the length of the lhs is equal to or shorter than the tab width or
        // the rhs looks like block expression, we put the rhs on the same
        // line with the lhs even if the rhs is multi-lined.
        let allow_same_line = lhs_result.len() <= tab_spaces
            || rhs_result
                .lines()
                .next()
                .map(|first_line| first_line.ends_with('{'))
                .unwrap_or(false);
        if !rhs_result.contains('\n') || allow_same_line {
            let one_line_width = last_line_width(&lhs_result)
                + pp.infix.len()
                + first_line_width(rhs_result)
                + pp.suffix.len();
            if one_line_width <= shape.width {
                return Some(format!(
                    "{}{}{}{}",
                    lhs_result, pp.infix, rhs_result, pp.suffix
                ));
            }
        }
    }

    // We have to use multiple lines.
    // Re-evaluate the rhs because we have more space now:
    let mut rhs_shape = match context.config.indent_style() {
        IndentStyle::Visual => shape
            .sub_width(pp.suffix.len() + pp.prefix.len())?
            .visual_indent(pp.prefix.len()),
        IndentStyle::Block => {
            // Try to calculate the initial constraint on the right hand side.
            let rhs_overhead = shape.rhs_overhead(context.config);
            Shape::indented(shape.indent.block_indent(context.config), context.config)
                .sub_width(rhs_overhead)?
        }
    };
    let infix = match separator_place {
        SeparatorPlace::Back => pp.infix.trim_end(),
        SeparatorPlace::Front => pp.infix.trim_start(),
    };
    if separator_place == SeparatorPlace::Front {
        rhs_shape = rhs_shape.offset_left(infix.len())?;
    }
    let rhs_result = rhs.rewrite(context, rhs_shape)?;
    let indent_str = rhs_shape.indent.to_string_with_newline(context.config);
    let infix_with_sep = match separator_place {
        SeparatorPlace::Back => format!("{}{}", infix, indent_str),
        SeparatorPlace::Front => format!("{}{}", indent_str, infix),
    };
    Some(format!(
        "{}{}{}{}",
        lhs_result, infix_with_sep, rhs_result, pp.suffix
    ))
}

// A pair which forms a tree and can be flattened (e.g., binops).
trait FlattenPair: Rewrite + Sized {
    fn flatten(&self, _: &RewriteContext<'_>, _: Shape) -> Option<PairList<'_, '_, Self>> {
        None
    }
}

struct PairList<'a, 'b, T: Rewrite> {
    list: Vec<(&'b T, Option<String>)>,
    separators: Vec<&'a str>,
    sep_prefixes: Vec<String>,
    sep_suffixes: Vec<String>,
}

impl FlattenPair for ast::Expr {
    fn flatten(
        &self,
        context: &RewriteContext<'_>,
        shape: Shape,
    ) -> Option<PairList<'_, '_, ast::Expr>> {
        let top_op = match self.kind {
            ast::ExprKind::Binary(op, _, _) => op.node,
            _ => return None,
        };

        let default_rewrite = |node: &ast::Expr, sep: usize, is_first: bool| {
            if is_first {
                return node.rewrite(context, shape);
            }
            let nested_overhead = sep + 1;
            let rhs_offset = shape.rhs_overhead(&context.config);
            let nested_shape = (match context.config.indent_style() {
                IndentStyle::Visual => shape.visual_indent(0),
                IndentStyle::Block => shape.block_indent(context.config.tab_spaces()),
            })
            .with_max_width(&context.config)
            .sub_width(rhs_offset)?;
            let default_shape = match context.config.binop_separator() {
                SeparatorPlace::Back => nested_shape.sub_width(nested_overhead)?,
                SeparatorPlace::Front => nested_shape.offset_left(nested_overhead)?,
            };
            node.rewrite(context, default_shape)
        };

        // Turn a tree of binop expressions into a list using a depth-first,
        // in-order traversal.
        let mut stack = vec![];
        let mut list = vec![];
        let mut separators = vec![];
        let mut node = self;
        let mut sep_prefixes = vec![]; /* pre separator comments */
        let mut sep_suffixes = vec![]; /* post separator comments */
        loop {
            match node.kind {
                ast::ExprKind::Binary(op, ref lhs, _) if op.node == top_op => {
                    stack.push(node);
                    node = lhs;
                }
                _ => {
                    let op_len = separators.last().map_or(0, |s: &&str| s.len());
                    let rw = default_rewrite(node, op_len, list.is_empty());
                    list.push((node, rw));
                    if let Some(pop) = stack.pop() {
                        match pop.kind {
                            ast::ExprKind::Binary(op, _, ref rhs) => {
                                separators.push(op.node.to_string());
                                /* Collect pre and post opertor comments */
                                let sp = mk_sp(node.span.hi(), op.span.lo());
                                let c = rewrite_missing_comment(sp, shape, context)?;
                                sep_prefixes.push(c.to_string());
                                let sp = mk_sp(op.span.hi(), rhs.span.lo());
                                let c = rewrite_missing_comment(sp, shape, context)?;
                                sep_suffixes.push(c.to_string());
                                node = rhs;
                            }
                            _ => unreachable!(),
                        }
                    } else {
                        break;
                    }
                }
            }
        }

        assert_eq!(list.len() - 1, separators.len());
        Some(PairList {
            list,
            separators,
            sep_prefixes,
            sep_suffixes,
        })
    }
}

impl FlattenPair for ast::Ty {}
impl FlattenPair for ast::Pat {}
