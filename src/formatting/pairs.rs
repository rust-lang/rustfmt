use rustc_ast::ast;
use std::borrow::Cow;

use crate::config::lists::*;
use crate::config::IndentStyle;
use crate::formatting::{
    comment::{
        combine_strs_with_comments, comment_style, rewrite_comment,
        rewrite_missing_comment_with_newline_start,
    },
    rewrite::{Rewrite, RewriteContext},
    shape::Shape,
    utils::{
        first_line_width, is_arithmetic_op, is_compare_op, is_single_line, last_line_width,
        longest_line_width, mk_sp, string_is_closing_brackets, trimmed_last_line_width, wrap_str,
    },
};

/// Sigils that decorate a binop pair.
#[derive(Clone, Copy)]
pub(crate) struct PairParts<'a> {
    prefix: &'a str,
    infix_prefix: &'a str, /* mainly for pre-infix comments */
    infix: &'a str,
    infix_suffix: &'a str, /* mainly for post-infix comments */
    suffix: &'a str,
}

impl<'a> PairParts<'a> {
    /// Constructs a new `PairParts`.
    pub(crate) fn new(
        prefix: &'a str,
        infix_prefix: &'a str,
        infix: &'a str,
        infix_suffix: &'a str,
        suffix: &'a str,
    ) -> Self {
        PairParts {
            prefix,
            infix_prefix,
            infix,
            infix_suffix,
            suffix,
        }
    }

    pub(crate) fn infix(infix: &'a str) -> PairParts<'a> {
        PairParts {
            prefix: "",
            infix_prefix: "",
            infix,
            infix_suffix: "",
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
    debug!("rewrite_pairs_one_line: {:?}", shape);
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
                if c.starts_with("//") || c.starts_with("\n") {
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
                if c.starts_with("\n") {
                    return None;
                }
                result.push_str(c);
                if c.starts_with("//") {
                    let rhs_offset = shape.rhs_overhead(&context.config);
                    let nested_shape = (match context.config.indent_style() {
                        IndentStyle::Visual => shape.visual_indent(0),
                        IndentStyle::Block => shape.block_indent(context.config.tab_spaces()),
                    })
                    .with_max_width(&context.config)
                    .sub_width(rhs_offset)?;
                    let indent_str = nested_shape.indent.to_string_with_newline(context.config);
                    result.push_str(&indent_str);
                } else {
                    result.push(' ');
                }
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
    debug!("rewrite_pairs_multiline: {:?}", shape);
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
    for ((e, default_rw), s) in list.list[1..].iter().zip(list.separators.iter()) {
        /* First multiline comment prefix and suffix in SeparatorPlace::Back
         * and that are added to existing line requires less alignment, compared to
         * the other comments that are added to already block aligned lines */
        let prefix_with_start = prefix_iter.next()?.trim_end();
        let prefix = prefix_with_start.trim_start();
        let suffix_with_start = suffix_iter.next()?.trim_end();
        let suffix = suffix_with_start.trim_start();
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

        // The following test checks if we should keep two subexprs on the same
        // line. We do this if not doing so would create an orphan and there is
        // enough space to do so.
        let offset = if result.contains('\n') {
            0
        } else {
            shape.used_width()
        };

        // Add pre-separator comment  - try to add at the end of current last line.
        let rw_prefix = if prelen <= 0 {
            String::new()
        } else {
            rewrite_comment(
                &prefix,
                true,
                if result.contains('\n') || prefix_with_start.starts_with("\n") {
                    nested_shape
                } else {
                    shape
                },
                context.config,
            )?
        };

        let width_for_one_line = last_line_width(&result) + offset + s.len() + 2;
        let line_shape = if width_for_one_line <= shape.used_width() {
            // We must snuggle the next line onto the previous line to avoid an orphan.
            shape.offset_left(s.len() + 2 + trimmed_last_line_width(&result))
        } else if width_for_one_line < shape.width && is_compare_op(s) {
            shape.offset_left(s.len() + 2)
        } else {
            nested_shape.offset_left(s.len() + 1)
        };

        let rw = if line_shape.is_some() {
            e.rewrite(context, line_shape?)?
        } else {
            String::new()
        };

        let rewrite = if !rw.is_empty() {
            &rw
        } else {
            &default_rw.as_ref()?
        };

        /* Add separator - in multiline each separator starts or ends a line */
        match context.config.binop_separator() {
            SeparatorPlace::Back => {
                let prefix_shape = if !result.contains('\n') {
                    shape
                } else {
                    nested_shape
                };
                // Combine comment with initial shape - including start new line if needed
                result = combine_strs_with_comments(
                    context,
                    &result,
                    "",
                    &rw_prefix,
                    prefix_shape,
                    0,
                    true,
                    !prefix_with_start.starts_with("\n"),
                    true,
                    true,
                )?;
                // Combine separator - using nested_shape in case it is in new line
                result = combine_strs_with_comments(
                    context,
                    &result,
                    s,
                    "",
                    nested_shape,
                    0,
                    true,
                    true,
                    true,
                    false,
                )?;

                let suffix_shape = if !result.contains('\n') {
                    shape
                } else {
                    nested_shape
                };
                let rw_suffix = if suflen <= 0 {
                    String::new()
                } else {
                    rewrite_comment(&suffix, true, suffix_shape, context.config)?
                };
                result = combine_strs_with_comments(
                    context,
                    &result,
                    &"",
                    &rw_suffix,
                    suffix_shape,
                    0,
                    true,
                    !suffix_with_start.starts_with("\n"),
                    true,
                    false,
                )?;
                result.push_str(&indent_str);
                result.push_str(&rewrite);
            }
            SeparatorPlace::Front => {
                let empty_cow = Cow::from("");
                // Find if result ensds with clocsing brackets line
                let result_last_line = result.lines().last().unwrap();
                let last_ends_with_closing_brackets =
                    result_last_line.rfind('}').map_or(false, |i| {
                        if string_is_closing_brackets(&result_last_line[i..], true) {
                            if i == 0 {
                                true
                            } else if result_last_line[..i - 1].trim().is_empty() {
                                true
                            } else {
                                false
                            }
                        } else {
                            false
                        }
                    });

                // Put compare operator in same line only if there are no comments
                // and all statement fits one line.
                let compare_op_in_same_line = is_compare_op(s)
                    && prelen == 0
                    && suflen == 0
                    // ????? && is_single_line(&rewrite)
                    && (is_single_line(&result)
                        || string_is_closing_brackets(result_last_line, false))
                    && trimmed_last_line_width(&result)
                        + s.len()
                        + 2
                        + rewrite.lines().next()?.len()
                        <= shape.width;
                let pre_sep_indent_str = if result.ends_with("}")
                    || (last_ends_with_closing_brackets && !is_arithmetic_op(s))
                    || compare_op_in_same_line
                {
                    &empty_cow
                } else {
                    &indent_str
                };

                let (prefix_shape, indentation_offset) = if is_single_line(&result) {
                    (shape, context.config.tab_spaces())
                } else {
                    (nested_shape, 0)
                };

                result = combine_strs_with_comments(
                    context,
                    &result,
                    &format!("{}{}", pre_sep_indent_str, s),
                    &rw_prefix,
                    prefix_shape,
                    indentation_offset,
                    true,
                    !prefix_with_start.starts_with("\n"),
                    true,
                    false,
                )?;

                let rw_suffix = if suflen <= 0 {
                    String::new()
                } else {
                    rewrite_comment(&suffix, true, nested_shape, context.config)?
                };

                let (suffix_shape, indentation_offset) = if !result.contains('\n') {
                    (shape, context.config.tab_spaces())
                } else {
                    (shape, context.config.tab_spaces())
                };

                result = combine_strs_with_comments(
                    context,
                    &result,
                    &rewrite,
                    &rw_suffix,
                    suffix_shape,
                    indentation_offset,
                    true,
                    !suffix_with_start.starts_with("\n"),
                    true,
                    false,
                )?;
            }
        }
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
    debug!("rewrite_pair: {:?}", shape);
    let tab_spaces = context.config.tab_spaces();

    // If infix_suffix is open-comment then a new line should be added to it.
    let c = pp.infix_suffix.trim_start();
    let rhs_infix_suffix =
        if !c.is_empty() && comment_style(c, false).is_line_comment() && !c.contains("\n") {
            format!(
                "{}{}",
                pp.infix_suffix.trim_end(),
                shape
                    .indent
                    .to_string_with_newline(context.config)
                    .to_string(),
            )
        } else {
            String::from(pp.infix_suffix)
        };

    let infix_result = format!("{}{}", pp.infix, rhs_infix_suffix);

    let lhs_overhead = match separator_place {
        SeparatorPlace::Back => {
            shape.used_width() + pp.prefix.len() + pp.infix.trim_end().len() + pp.infix_prefix.len()
        }
        SeparatorPlace::Front => shape.used_width(),
    };
    let lhs_shape = Shape {
        width: context.budget(lhs_overhead),
        ..shape
    };

    // If infix_prefix is open-comment then a new line should be added to it.
    let c = pp.infix_prefix.trim_start();
    let lhs_infix_suffix =
        if !c.is_empty() && comment_style(c, false).is_line_comment() && !c.contains("\n") {
            format!(
                "{}{}",
                pp.infix_prefix.trim_end(),
                shape
                    .indent
                    .to_string_with_newline(context.config)
                    .to_string(),
            )
        } else {
            String::from(pp.infix_prefix)
        };

    let lhs_result = lhs
        .rewrite(context, lhs_shape)
        .map(|lhs_str| format!("{}{}{}", pp.prefix, lhs_str, lhs_infix_suffix))?;

    // Try to put both lhs and rhs on the same line.
    let rhs_orig_result = shape
        .offset_left(last_line_width(&lhs_result) + pp.infix.len())
        .and_then(|s| s.sub_width(pp.suffix.len() + pp.infix_suffix.len()))
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
                + infix_result.len()
                + first_line_width(rhs_result)
                + pp.suffix.len();
            if one_line_width <= shape.width {
                return Some(format!(
                    "{}{}{}{}",
                    lhs_result, infix_result, rhs_result, pp.suffix
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
        SeparatorPlace::Front => {
            if pp.infix_suffix.is_empty() {
                pp.infix.trim_start()
            } else {
                pp.infix
            }
        }
    };
    let infix_suffix = if separator_place == SeparatorPlace::Front && !pp.infix_suffix.is_empty() {
        pp.infix_suffix.trim_start()
    } else {
        pp.infix_suffix
    };

    if separator_place == SeparatorPlace::Front {
        rhs_shape = rhs_shape.offset_left(infix.len())?;
    }
    let rhs_result = rhs.rewrite(context, rhs_shape)?;
    let indent_str = rhs_shape.indent.to_string_with_newline(context.config);
    let (mut infix_with_sep, infix_with_sep_len, max_width) = match separator_place {
        SeparatorPlace::Back => {
            let s = format!("{}{}", infix, infix_suffix.trim_end());
            (format!("{}{}", s, indent_str), s.len(), shape.width)
        }
        SeparatorPlace::Front => {
            let new_indent_str = if string_is_closing_brackets(lhs_result.lines().last()?, true) {
                Cow::from(" ")
            } else {
                indent_str.clone()
            };
            let s = format!("{}{}{}", new_indent_str, infix.trim_start(), infix_suffix);
            (s.clone(), s.len() - 1, context.config.max_width())
        }
    };

    let new_line_width = infix_with_sep_len + rhs_result.len() + pp.suffix.len();
    let rhs_with_sep = if separator_place == SeparatorPlace::Front && new_line_width > max_width {
        let s: String = String::from(infix_with_sep);
        infix_with_sep = s.trim_end().to_string();
        format!("{}{}", indent_str, rhs_result.trim_start())
    } else {
        rhs_result
    };

    Some(format!(
        "{}{}{}{}",
        lhs_result, infix_with_sep, rhs_with_sep, pp.suffix
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
        debug!("FlattenPair: {:?} {:?}", shape, self);
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
                                // Collect pre and post opertor comments.
                                let sp = mk_sp(node.span.hi(), op.span.lo());
                                let c =
                                    rewrite_missing_comment_with_newline_start(sp, shape, context)?;
                                sep_prefixes.push(c.to_string());
                                let sp = mk_sp(op.span.hi(), rhs.span.lo());
                                let c =
                                    rewrite_missing_comment_with_newline_start(sp, shape, context)?;
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
