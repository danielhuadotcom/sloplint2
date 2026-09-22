//! Comment & docstring rules.
//!
//! - `SLP010` comment policy — comments banned by default (stable).
//! - `SLP011` docstring policy — docstrings banned (stable).
//! - `SLP012` noqa policy — `# noqa` banned (stable).
//! - `SLP050` ASCII-only source (stable).
//! - `SLP001` redundant "what" comment (preview — heuristic).
//! - `SLP002` redundant docstring (preview — heuristic).

pub mod ascii_only;
pub mod comment_policy;
pub mod comment_tells;
pub mod docstring_policy;
pub mod noqa_policy;
pub mod redundant_comment;
pub mod redundant_docstring;

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_rule;

    test_rule!(
        slp010_comment_policy,
        comment_policy::CommentPolicy,
        "comments",
        "SLP010"
    );
    test_rule!(
        slp011_docstring_policy,
        docstring_policy::DocstringPolicy,
        "comments",
        "SLP011"
    );
    test_rule!(
        slp012_noqa_policy,
        noqa_policy::NoqaPolicy,
        "comments",
        "SLP012"
    );
    test_rule!(
        slp050_ascii_only,
        ascii_only::AsciiOnly,
        "comments",
        "SLP050"
    );
    test_rule!(
        slp001_redundant_comment,
        redundant_comment::RedundantComment,
        "comments",
        "SLP001"
    );
    test_rule!(
        slp002_redundant_docstring,
        redundant_docstring::RedundantDocstring,
        "comments",
        "SLP002"
    );
    test_rule!(
        slp004_comment_tells,
        comment_tells::CommentTells,
        "comments",
        "SLP004"
    );
}
