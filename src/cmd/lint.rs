use mit_commit::CommitMessage;

use crate::model::{Lints, Problem};

#[must_use]
pub fn lint(commit_message: &CommitMessage, lints: Lints) -> Vec<Problem> {
    lints
        .into_iter()
        .collect::<Vec<_>>()
        .into_iter()
        .filter_map(|lint| lint.lint(commit_message))
        .collect::<Vec<Problem>>()
}