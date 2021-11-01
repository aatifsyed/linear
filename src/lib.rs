#![feature(rustc_private)]
#![warn(unused_extern_crates)]

dylint_linting::dylint_library!();

extern crate rustc_hir;
extern crate rustc_lint;
extern crate rustc_session;

mod linear_lint;

#[doc(hidden)]
#[no_mangle]
pub fn register_lints(_sess: &rustc_session::Session, lint_store: &mut rustc_lint::LintStore) {
    lint_store.register_lints(&[linear_lint::LINEAR]);
    lint_store.register_late_pass(|| Box::new(linear_lint::FillMeIn));
}

pub trait Linear {
    const HINT: &'static str = "This type has been marked as undroppable. Consult the documentation for destruction instructions.";
}

struct Foo;
impl Linear for Foo {}
