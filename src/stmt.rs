use rustc_ast::ast;
use rustc_span::Span;

use crate::comment::recover_comment_removed;
use crate::config::Version;
use crate::expr::{contains_curly_block, format_expr, is_simple_block, ExprType};
use crate::rewrite::{Rewrite, RewriteContext};
use crate::shape::Shape;
use crate::source_map::LineRangeUtils;
use crate::spanned::Spanned;
use crate::utils::semicolon_for_stmt;

pub(crate) struct Stmt<'a> {
    inner: &'a ast::Stmt,
    is_first: bool,
    is_last: bool,
}

impl<'a> Spanned for Stmt<'a> {
    fn span(&self) -> Span {
        self.inner.span()
    }
}

impl<'a> Stmt<'a> {
    pub(crate) fn as_ast_node(&self) -> &ast::Stmt {
        self.inner
    }

    pub(crate) fn to_item(&self) -> Option<&ast::Item> {
        match self.inner.kind {
            ast::StmtKind::Item(ref item) => Some(&**item),
            _ => None,
        }
    }

    pub(crate) fn from_simple_block(
        context: &RewriteContext<'_>,
        block: &'a ast::Block,
        attrs: Option<&[ast::Attribute]>,
    ) -> Option<Self> {
        if is_simple_block(context, block, attrs) {
            let inner = &block.stmts[0];
            // Simple blocks only contain one expr and no stmts
            let is_first = true;
            let is_last = true;
            Some(Stmt {
                inner,
                is_first,
                is_last,
            })
        } else {
            None
        }
    }

    pub(crate) fn from_ast_node(inner: &'a ast::Stmt, is_first: bool, is_last: bool) -> Self {
        Stmt {
            inner,
            is_first,
            is_last,
        }
    }

    pub(crate) fn from_ast_nodes<I>(iter: I) -> Vec<Self>
    where
        I: Iterator<Item = &'a ast::Stmt>,
    {
        let mut result = vec![];
        let mut iter = iter.peekable();

        if let Some(inner) = iter.next() {
            result.push(Stmt {
                inner,
                is_first: true,
                is_last: iter.peek().is_none(),
            })
        }

        while let Some(inner) = iter.next() {
            result.push(Stmt {
                inner,
                is_first: false,
                is_last: iter.peek().is_none(),
            })
        }
        result
    }

    pub(crate) fn is_empty(&self) -> bool {
        matches!(self.inner.kind, ast::StmtKind::Empty)
    }

    pub(crate) fn is_first(&self) -> bool {
        self.is_first
    }

    pub(crate) fn is_last(&self) -> bool {
        self.is_last
    }

    fn is_last_expr(&self) -> bool {
        if !self.is_last {
            return false;
        }

        match self.as_ast_node().kind {
            ast::StmtKind::Expr(ref expr) => match expr.kind {
                ast::ExprKind::Ret(..) | ast::ExprKind::Continue(..) | ast::ExprKind::Break(..) => {
                    false
                }
                _ => true,
            },
            _ => false,
        }
    }

    pub(crate) fn is_block_with_curly_braces(&self) -> bool {
        match self.as_ast_node().kind {
            ast::StmtKind::Let(ref local) => match local.kind {
                ast::LocalKind::Decl => false,
                ast::LocalKind::Init(ref expr) => contains_curly_block(expr),
                ast::LocalKind::InitElse(..) => true,
            },
            ast::StmtKind::Expr(ref expr) | ast::StmtKind::Semi(ref expr) => {
                contains_curly_block(expr)
            }
            _ => false,
        }
    }
}

impl<'a> Rewrite for Stmt<'a> {
    fn rewrite(&self, context: &RewriteContext<'_>, shape: Shape) -> Option<String> {
        let expr_type = if context.config.version() == Version::Two && self.is_last_expr() {
            ExprType::SubExpression
        } else {
            ExprType::Statement
        };
        format_stmt(
            context,
            shape,
            self.as_ast_node(),
            expr_type,
            self.is_last_expr(),
        )
    }
}

fn format_stmt(
    context: &RewriteContext<'_>,
    shape: Shape,
    stmt: &ast::Stmt,
    expr_type: ExprType,
    is_last_expr: bool,
) -> Option<String> {
    skip_out_of_file_lines_range!(context, stmt.span());

    let result = match stmt.kind {
        ast::StmtKind::Let(ref local) => local.rewrite(context, shape),
        ast::StmtKind::Expr(ref ex) | ast::StmtKind::Semi(ref ex) => {
            let suffix = if semicolon_for_stmt(context, stmt, is_last_expr) {
                ";"
            } else {
                ""
            };

            let shape = shape.sub_width(suffix.len())?;
            format_expr(ex, expr_type, context, shape).map(|s| s + suffix)
        }
        ast::StmtKind::MacCall(..) | ast::StmtKind::Item(..) | ast::StmtKind::Empty => None,
    };
    result.and_then(|res| recover_comment_removed(res, stmt.span(), context))
}
