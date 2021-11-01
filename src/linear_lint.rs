use clippy_utils::diagnostics::span_lint_and_help;
use clippy_utils::{is_ty_param_lang_item, match_function_call};
use rustc_hir::Expr;
use rustc_lint::{LateContext, LateLintPass};
use rustc_session::{declare_lint, declare_lint_pass};

declare_lint! {
    /// **What it does:**
    ///
    /// **Why is this bad?**
    ///
    /// **Known problems:** None.
    ///
    /// **Example:**
    ///
    /// ```rust
    /// // example code where a warning is issued
    /// ```
    /// Use instead:
    /// ```rust
    /// // example code that does not raise a warning
    /// ```
    pub LINEAR,
    Forbid,
    "description goes here"
}

declare_lint_pass!(FillMeIn => [LINEAR]);

impl<'hir> LateLintPass<'hir> for FillMeIn {
    // A list of things you might check can be found here:
    // https://doc.rust-lang.org/stable/nightly-rustc/rustc_lint/trait.LateLintPass.html
    fn check_expr(&mut self, cx: &LateContext<'hir>, expr: &'hir Expr<'_>) {
        println!("called");
        if let Some(args) = match_function_call(cx, expr, &["core", "mem", "drop"]) {
            span_lint_and_help(
                cx,
                LINEAR,
                expr.span,
                "This is a lint",
                None,
                "Call the ultimate consumer for this type",
            )
        }
    }
}
