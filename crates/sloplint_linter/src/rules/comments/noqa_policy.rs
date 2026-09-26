//! SLP012: lint suppression directives are banned.

use sloplint_diagnostics::{Diagnostic, Severity};
use sloplint_python::TextRange;

use crate::lint::{FileContext, Rule};
use crate::suppression::NOQA_BAN_CODE;
use sloplint_macros::ViolationMetadata;

/// Ruff's range and file suppression actions; each takes a bracketed code list.
const RUFF_ACTIONS: &[&str] = &["disable", "ignore", "file-ignore"];

/// ## What it does
/// Flags every comment that turns a lint check off, in the forms Ruff recognizes: an inline
/// `# noqa` (whatever codes it names), a file-level `# ruff: noqa` or `# flake8: noqa`, a
/// `# ruff: disable[...]`, `# ruff: ignore[...]` or `# ruff: file-ignore[...]` suppression, and
/// isort's `skip`, `skip_file` and `off` actions. This rule can never be suppressed by a `# noqa`,
/// since a directive that excused its own ban would make the rule vacuous; silence it through
/// `ignore` or a per-path override in config.
///
/// ## Why is this bad?
/// A suppression turns a finding off where it was proven to apply, and it outlives the reason it
/// was added: the code around it changes, the original defect returns, and the directive keeps the
/// linter quiet. File-level and range forms go further and hide every future finding in their
/// scope. Ruff's `RUF100` only reports a `# noqa` that suppresses nothing, so no upstream rule bans
/// the directives themselves.
///
/// ## Example
/// ```python
/// # ruff: noqa: E501
/// value = compute()  # noqa: SLP030
/// ```
#[derive(ViolationMetadata)]
pub struct NoqaPolicy;

impl Rule for NoqaPolicy {
    fn code(&self) -> &'static str {
        NOQA_BAN_CODE
    }

    fn check_comment(
        &self,
        ctx: &FileContext,
        range: TextRange,
        diagnostics: &mut Vec<Diagnostic>,
    ) {
        if !is_suppression(&ctx.source[range]) {
            return;
        }
        diagnostics.push(Diagnostic::new(
            self.code(),
            "suppression directive is not allowed (lint suppressions are banned)",
            range,
            Severity::Warning,
        ));
    }
}

/// Whether a comment carries a suppression directive. Ruff reads directives after any `#` in the
/// comment (`# type: ignore # noqa`), so each `#`-delimited segment is checked on its own.
fn is_suppression(comment: &str) -> bool {
    comment
        .split('#')
        .skip(1)
        .any(|segment| is_directive(segment.trim_start()))
        || is_isort_skip(comment)
}

fn is_directive(text: &str) -> bool {
    if is_noqa(text) {
        return true;
    }
    if after_tool(text, "flake8").is_some_and(is_noqa) {
        return true;
    }
    let Some(rest) = after_tool(text, "ruff") else {
        return false;
    };
    is_noqa(rest)
        || RUFF_ACTIONS.iter().any(|action| {
            rest.strip_prefix(action)
                .is_some_and(|codes| codes.trim_start().starts_with('['))
        })
}

/// The text after a `tool:` prefix, with whitespace allowed around the colon.
fn after_tool<'a>(text: &'a str, tool: &str) -> Option<&'a str> {
    let rest = text.strip_prefix(tool)?.trim_start().strip_prefix(':')?;
    Some(rest.trim_start())
}

/// A case-insensitive `noqa` ending the segment or followed by whitespace or `:`, so `noqaX`
/// stays prose.
fn is_noqa(text: &str) -> bool {
    let Some(keyword) = text.get(..4) else {
        return false;
    };
    keyword.eq_ignore_ascii_case("noqa")
        && text[4..]
            .chars()
            .next()
            .is_none_or(|c| c == ':' || c.is_whitespace())
}

/// Ruff's isort actions match on the whole comment: `skip` and `skip_file` anywhere in it, `off`
/// only as the entire comment.
fn is_isort_skip(comment: &str) -> bool {
    let comment = comment.trim_end();
    comment.contains("isort: skip")
        || comment.contains("isort:skip")
        || matches!(comment, "# isort: off" | "# ruff: isort: off")
}
