use clippy_utils::diagnostics::span_lint_and_help;
use clippy_utils::{is_ty_param_lang_item, match_function_call};
use rustc_hir::Block;
use rustc_lint::{LateContext, LateLintPass};
use rustc_middle::ty::WithOptConstParam;
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
    fn check_block(&mut self, cx: &LateContext<'hir>, expr: &'hir Block<'_>) {
        let (thir, _expr_id) = cx.tcx.thir_body(WithOptConstParam {
            did: cx.last_node_with_lint_attrs.owner,
            const_param_did: None,
        });
        let thir = thir.borrow();
    }
}
