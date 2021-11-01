use std::option::Option::None;

use mit_commit::CommitMessage;

use crate::model::{Code, Problem};

/// Canonical lint ID
pub const CONFIG: &str = "subject-line-ends-with-period";
/// Description of the problem
pub const ERROR: &str = "Your commit message ends with a period";
/// Advice on how to correct the problem
pub const HELP_MESSAGE: &str = "It's important to keep your commits short, because we only have a \
                            limited number of characters to use (72) before the subject line is \
                            truncated. Full stops aren't normally in subject lines, and take up \
                            an extra character, so we shouldn't use them in commit message \
                            subjects.\n\nYou can fix this by removing the period";

fn has_problem(commit_message: &CommitMessage<'_>) -> bool {
    commit_message
        .get_subject()
        .to_string()
        .trim_end()
        .chars()
        .rev()
        .next()
        .filter(|x| *x == '.')
        .is_some()
}

pub fn lint(commit_message: &CommitMessage<'_>) -> Option<Problem> {
    if has_problem(commit_message) {
        let subject = commit_message.get_subject().to_string();
        Some(Problem::new(
            ERROR.into(),
            HELP_MESSAGE.into(),
            Code::SubjectEndsWithPeriod,
            commit_message,
            Some(vec![(
                "Unneeded period".to_string(),
                subject.len()
                    - subject
                        .chars()
                        .rev()
                        .filter(|ch| ch == &'.' || ch.is_whitespace())
                        .count()
                        .saturating_sub(2),
                subject
                    .chars()
                    .rev()
                    .filter(|ch| !ch.is_whitespace())
                    .take_while(|ch| ch == &'.')
                    .count(),
            )]),
            Some("https://git-scm.com/book/en/v2/Distributed-Git-Contributing-to-a-Project#_commit_guidelines".parse().unwrap()),
        ))
    } else {
        None
    }
}
