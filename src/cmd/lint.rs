use mit_commit::CommitMessage;

use crate::model::{Lints, Problem};

/// Lint a commit message
///
/// Example
///
/// ```rust
/// use mit_commit::CommitMessage;
/// use mit_lint::Lints;
/// use mit_lint::lint;
/// let actual = lint(&CommitMessage::from("An example commit message"), Lints::available().clone());
/// assert!(!actual.is_empty());
/// ```

#[must_use]
pub fn lint(commit_message: &CommitMessage, lints: Lints) -> Vec<Problem> {
    lints
        .into_iter()
        .collect::<Vec<_>>()
        .into_iter()
        .filter_map(|lint| lint.lint(commit_message))
        .collect::<Vec<Problem>>()
}
