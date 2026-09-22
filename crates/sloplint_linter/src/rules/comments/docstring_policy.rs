//! SLP011: docstrings are banned.

use sloplint_diagnostics::{Diagnostic, Severity};
use sloplint_python::ast::Stmt;
use sloplint_python::docstring_range;

use crate::lint::{FileContext, Rule};
use sloplint_macros::ViolationMetadata;

/// ## What it does
/// Flags every docstring — module, class, function, and method — extending the `SLP010` comment
/// ban to the one form of prose Python gives a syntactic home.
///
/// ## Why is this bad?
/// A docstring is prose about code, and carries the same drift and narration problems as a
/// comment: it restates the signature, goes stale when the code changes, and is a favored
/// vehicle for generated filler. Ruff's pydocstyle rules enforce the opposite policy (`D1xx`
/// *requires* docstrings), so nothing upstream covers this.
///
/// ## Example
/// ```python
/// def parse(text):
///     """Parse the text and return the result."""
///     return _parse(text)
/// ```
#[derive(ViolationMetadata)]
pub struct DocstringPolicy;

impl Rule for DocstringPolicy {
    fn code(&self) -> &'static str {
        "SLP011"
    }

    fn check_stmt(&self, stmt: &Stmt, _ctx: &FileContext, diagnostics: &mut Vec<Diagnostic>) {
        let body = match stmt {
            Stmt::FunctionDef(node) => &node.body,
            Stmt::ClassDef(node) => &node.body,
            _ => return,
        };
        if let Some(range) = docstring_range(body) {
            diagnostics.push(Diagnostic::new(
                self.code(),
                "docstring is not allowed (docstrings are banned)",
                range,
                Severity::Warning,
            ));
        }
    }

    /// The module docstring has no enclosing statement, so it is picked up here rather than in
    /// the statement walk.
    fn check_source(&self, ctx: &FileContext, diagnostics: &mut Vec<Diagnostic>) {
        if let Some(range) = docstring_range(&ctx.parsed.syntax().body) {
            diagnostics.push(Diagnostic::new(
                self.code(),
                "module docstring is not allowed (docstrings are banned)",
                range,
                Severity::Warning,
            ));
        }
    }
}
