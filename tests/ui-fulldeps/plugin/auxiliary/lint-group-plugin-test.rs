// force-host

#![feature(rustc_private)]

// Load rustc as a plugin to get macros.
extern crate rustc_driver;
extern crate rustc_hir;
extern crate rustc_lint;
#[macro_use]
extern crate rustc_session;

use rustc_driver::plugin::Registry;
use rustc_lint::{LateContext, LateLintPass, LintContext, LintId};

declare_lint!(TEST_LINT, Warn, "Warn about items named 'lintme'");

declare_lint!(PLEASE_LINT, Warn, "Warn about items named 'pleaselintme'");

declare_lint_pass!(Pass => [TEST_LINT, PLEASE_LINT]);

impl<'tcx> LateLintPass<'tcx> for Pass {
    fn check_item(&mut self, cx: &LateContext, it: &rustc_hir::Item) {
        match it.ident.as_str() {
            "lintme" => cx.lint(TEST_LINT, "item is named 'lintme'", |lint| lint.set_span(it.span)),
            "pleaselintme" => {
                cx.lint(PLEASE_LINT, "item is named 'pleaselintme'", |lint| lint.set_span(it.span))
            }
            _ => {}
        }
    }
}

#[no_mangle]
fn __rustc_plugin_registrar(reg: &mut Registry) {
    reg.lint_store.register_lints(&[&TEST_LINT, &PLEASE_LINT]);
    reg.lint_store.register_late_pass(|_| Box::new(Pass));
    reg.lint_store.register_group(
        true,
        "lint_me",
        None,
        vec![LintId::of(&TEST_LINT), LintId::of(&PLEASE_LINT)],
    );
}
