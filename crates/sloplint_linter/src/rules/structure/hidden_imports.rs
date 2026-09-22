//! SLP181: imports hidden from static top-level analysis.

use sloplint_diagnostics::{Diagnostic, Severity};
use sloplint_python::ast::visitor::{self, Visitor};
use sloplint_python::ast::{ExceptHandler, Expr, Stmt};
use sloplint_python::Ranged;

use crate::lint::{FileContext, Rule};
use sloplint_macros::ViolationMetadata;

/// ## What it does
/// Flags every import that a reader of the module's first lines would not see: an import nested
/// inside a block (`if`, `try`, `with`, `for`, `while`, `match`), an import inside a function or
/// class body, and a dynamic import (`__import__(...)`, `importlib.import_module(...)`) anywhere
/// in the file. Only a plain `import` / `from ... import` at the top level of the module passes.
///
/// ## Why is this bad?
/// An import that only a runtime path reaches defeats every static consumer of the import graph:
/// dependency scanners, unused-import detection, circular-import analysis, and sloplint's own
/// `SLP180`. A dynamic import additionally takes its module name from a value, so the dependency
/// is not recoverable from the source at all.
///
/// ## Example
/// ```python
/// if sys.platform == "win32":
///     import winreg           # nested in a block
///
///
/// def load(name):
///     import json             # deferred into a function body
///     return importlib.import_module(name)   # dynamic
/// ```
#[derive(ViolationMetadata)]
pub struct HiddenImports;

impl Rule for HiddenImports {
    fn code(&self) -> &'static str {
        "SLP181"
    }

    fn check_source(&self, ctx: &FileContext, diagnostics: &mut Vec<Diagnostic>) {
        for stmt in &ctx.parsed.syntax().body {
            hidden_imports(stmt, false, self.code(), diagnostics);
        }
        let mut finder = DynamicImports {
            code: self.code(),
            diagnostics,
        };
        finder.visit_body(&ctx.parsed.syntax().body);
    }
}

/// Report every `import` / `from ... import` below the top level of the module, descending
/// through blocks and into function and class bodies alike. `deferred` records whether the
/// statements being scanned sit inside a function or class, which is all that distinguishes the
/// two messages.
fn hidden_imports(
    stmt: &Stmt,
    deferred: bool,
    code: &'static str,
    diagnostics: &mut Vec<Diagnostic>,
) {
    for (body, scope) in sub_bodies(stmt) {
        let deferred = deferred || scope;
        for inner in body {
            if matches!(inner, Stmt::Import(_) | Stmt::ImportFrom(_)) {
                let message = if deferred {
                    "import inside a function or class body is not allowed \
                     (imports must be plain statements at module top level)"
                } else {
                    "import nested inside a block is not allowed \
                     (imports must be plain statements at module top level)"
                };
                diagnostics.push(Diagnostic::new(
                    code,
                    message,
                    inner.range(),
                    Severity::Warning,
                ));
            }
            hidden_imports(inner, deferred, code, diagnostics);
        }
    }
}

/// The sub-bodies of a compound statement, each paired with whether it opens a new function or
/// class scope; nothing for a simple statement.
fn sub_bodies(stmt: &Stmt) -> Vec<(&[Stmt], bool)> {
    match stmt {
        Stmt::FunctionDef(node) => vec![(node.body.as_slice(), true)],
        Stmt::ClassDef(node) => vec![(node.body.as_slice(), true)],
        Stmt::If(node) => {
            let mut bodies = vec![(node.body.as_slice(), false)];
            bodies.extend(
                node.elif_else_clauses
                    .iter()
                    .map(|c| (c.body.as_slice(), false)),
            );
            bodies
        }
        Stmt::For(node) => vec![
            (node.body.as_slice(), false),
            (node.orelse.as_slice(), false),
        ],
        Stmt::While(node) => vec![
            (node.body.as_slice(), false),
            (node.orelse.as_slice(), false),
        ],
        Stmt::With(node) => vec![(node.body.as_slice(), false)],
        Stmt::Try(node) => {
            let mut bodies = vec![
                (node.body.as_slice(), false),
                (node.orelse.as_slice(), false),
                (node.finalbody.as_slice(), false),
            ];
            bodies.extend(node.handlers.iter().map(|handler| {
                let ExceptHandler::ExceptHandler(handler) = handler;
                (handler.body.as_slice(), false)
            }));
            bodies
        }
        Stmt::Match(node) => node
            .cases
            .iter()
            .map(|case| (case.body.as_slice(), false))
            .collect(),
        _ => Vec::new(),
    }
}

/// Finds dynamic-import calls anywhere in the tree, at any scope.
struct DynamicImports<'a> {
    code: &'static str,
    diagnostics: &'a mut Vec<Diagnostic>,
}

impl<'a> Visitor<'a> for DynamicImports<'_> {
    fn visit_expr(&mut self, expr: &'a Expr) {
        if let Expr::Call(call) = expr {
            if let Some(name) = dynamic_import_name(&call.func) {
                self.diagnostics.push(Diagnostic::new(
                    self.code,
                    format!("dynamic import `{name}` is not allowed (dynamic imports are banned)"),
                    call.range(),
                    Severity::Warning,
                ));
            }
        }
        visitor::walk_expr(self, expr);
    }
}

/// The name of the dynamic-import callable `func` refers to, or `None`. Matches the bare builtin
/// and the attribute forms (`importlib.import_module`, `importlib.__import__`) by their final
/// segment, so an aliased or re-exported `importlib` still resolves.
fn dynamic_import_name(func: &Expr) -> Option<&'static str> {
    let segment = match func {
        Expr::Name(name) => name.id.as_str(),
        Expr::Attribute(attribute) => attribute.attr.as_str(),
        _ => return None,
    };
    match segment {
        "__import__" => Some("__import__"),
        "import_module" => Some("import_module"),
        _ => None,
    }
}
