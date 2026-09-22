//! SLP070: test code is banned.

use sloplint_diagnostics::{Diagnostic, Severity};
use sloplint_python::ast::Stmt;
use sloplint_python::Ranged;

use crate::lint::{FileContext, Rule};
use sloplint_macros::ViolationMetadata;

/// Test frameworks whose import marks a file as test code. Matched on the import's root package,
/// so `unittest.mock` and `pytest.fixture` are covered by their root.
const FRAMEWORKS: &[&str] = &["pytest", "unittest", "nose", "nose2"];

/// ## What it does
/// Flags test code: an import of a test framework (`pytest`, `unittest`, `nose`, `nose2`), a
/// function named `test_*`, and a class named `Test*`.
///
/// ## Why is this bad?
/// Where a project's policy is that tests do not live in the tree, their presence is the defect,
/// and no mainstream linter reports it — Ruff's `flake8-pytest-style` and `flake8-bandit` rules
/// assume tests exist and only police how they are written.
///
/// ## Example
/// ```python
/// import pytest
///
///
/// def test_parses_empty_input():
///     assert parse("") == []
/// ```
#[derive(ViolationMetadata)]
pub struct NoTests;

impl Rule for NoTests {
    fn code(&self) -> &'static str {
        "SLP070"
    }

    fn check_stmt(&self, stmt: &Stmt, _ctx: &FileContext, diagnostics: &mut Vec<Diagnostic>) {
        let finding = match stmt {
            Stmt::Import(node) => node
                .names
                .iter()
                .find(|alias| is_framework(&alias.name))
                .map(|alias| format!("importing test framework `{}`", root_package(&alias.name))),
            Stmt::ImportFrom(node) => node
                .module
                .as_ref()
                .filter(|module| is_framework(module))
                .map(|module| format!("importing test framework `{}`", root_package(module))),
            Stmt::FunctionDef(node) if is_test_name(&node.name) => {
                Some(format!("test function `{}`", node.name))
            }
            Stmt::ClassDef(node) if is_test_class(&node.name) => {
                Some(format!("test class `{}`", node.name))
            }
            _ => return,
        };
        if let Some(what) = finding {
            diagnostics.push(Diagnostic::new(
                self.code(),
                format!("{what} is not allowed (test code is banned)"),
                stmt.range(),
                Severity::Warning,
            ));
        }
    }
}

/// The part of a dotted import path before the first `.` — `unittest.mock` resolves to `unittest`.
fn root_package(module: &str) -> &str {
    module.split('.').next().unwrap_or(module)
}

fn is_framework(module: &str) -> bool {
    FRAMEWORKS.contains(&root_package(module))
}

/// `test_foo` and the bare name `test`, matching pytest's own collection rule. A word boundary
/// keeps `testable_values` — a normal function that merely starts with the letters — out.
fn is_test_name(name: &str) -> bool {
    name == "test" || name.starts_with("test_")
}

/// `Test`, `TestCase`, `TestParser` — pytest collects a class whose name starts with `Test`. The
/// next character must not be lowercase, so `Tested` / `Testable` stay out.
fn is_test_class(name: &str) -> bool {
    let Some(rest) = name.strip_prefix("Test") else {
        return false;
    };
    !rest.chars().next().is_some_and(char::is_lowercase)
}
