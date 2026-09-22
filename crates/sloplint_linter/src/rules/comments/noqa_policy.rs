//! SLP012: `# noqa` is banned.

use sloplint_diagnostics::{Diagnostic, Severity};
use sloplint_python::TextRange;

use crate::lint::{FileContext, Rule};
use crate::suppression::{is_noqa, NOQA_BAN_CODE};
use sloplint_macros::ViolationMetadata;

/// ## What it does
/// Flags every `# noqa` directive, whatever codes it names — sloplint's own, Ruff's, or a blanket
/// one. This rule can never be suppressed by a `# noqa`, since a directive that excused its own
/// ban would make the rule vacuous; silence it through `ignore` or a per-path override in config.
///
/// ## Why is this bad?
/// An inline suppression turns a finding off at the one site where it was proven to apply, and it
/// outlives the reason it was added: the code around it changes, the original defect returns, and
/// the directive keeps the linter quiet. Ruff's `RUF100` only reports a `# noqa` that suppresses
/// nothing, so no upstream rule bans the directive itself.
///
/// ## Example
/// ```python
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
        if !is_noqa(&ctx.source[range]) {
            return;
        }
        diagnostics.push(Diagnostic::new(
            self.code(),
            "`# noqa` is not allowed (noqa directives are banned)",
            range,
            Severity::Warning,
        ));
    }
}
