use rustc::lint::in_external_macro;
use rustc_lint::{EarlyContext, EarlyLintPass, LintContext};
use rustc_session::{declare_lint_pass, declare_tool_lint};
use syntax::ast::*;

use crate::utils::span_help_and_lint;

declare_clippy_lint! {
    /// **What it does:** Checks for redundant uses of `continue` in loops.
    ///
    /// **Why is this bad?** Redundant `continue`s can mislead readers of the code into
    /// thinking the code requires a `continue`, which can lead to false assumptions
    /// about the underlying logic, especially in loops with long bodies.
    ///
    /// **Known problems:** None.
    ///
    /// **Example:**
    ///
    /// ```rust
    /// let mut x = 0;
    ///
    /// loop {
    ///     if x == 0 {
    ///         x += 1;
    ///     } else {
    ///         x += 2;
    ///     }
    ///     continue
    /// }
    /// ```
    ///
    /// Could be written:
    ///
    /// ```rust
    /// loop {
    ///     if x == 0 {
    ///         x += 1;
    ///     } else {
    ///         x += 2;
    ///     }
    /// }
    /// ```
    pub REDUNDANT_CONTINUE,
    style,
    "Redundant `continue`s inside loops"
}

declare_lint_pass!(RedundantContinue => [REDUNDANT_CONTINUE]);

impl EarlyLintPass for RedundantContinue {
    fn check_expr(&mut self, cx: &EarlyContext<'_>, item: &Expr) {
        if in_external_macro(cx.sess(), item.span) {
            return;
        }

        if let ExprKind::Loop(ref body, _) = item.kind {
            check_loop_body(cx, &body);
        }
    }
}

fn check_loop_body(cx: &EarlyContext<'_>, body: &Block) {
    // detect continue as final statement in body
    if let Some(stmt) = body.stmts.last() {
        if let StmtKind::Expr(ref expr) = stmt.kind {
            if let ExprKind::Continue(_) = expr.kind {
                span_help_and_lint(
                    cx,
                    REDUNDANT_CONTINUE,
                    expr.span,
                    "loop with trailing `continue`",
                    "consider removing",
                );
            }
        }
    }
}
