use std::option::Option::None;

use mit_commit::CommitMessage;

use crate::model::{Code, Problem};

/// Canonical lint ID
pub const CONFIG: &str = "subject-line-not-capitalized";
/// Advice on how to correct the problem
pub const HELP_MESSAGE: &str = "The subject line is a title, and as such should be \
                            capitalised.\n\nYou can fix this by capitalising the first character \
                            in the subject";
/// Description of the problem
pub const ERROR: &str = "Your commit message is missing a capital letter";

fn has_problem(commit_message: &CommitMessage<'_>) -> bool {
    commit_message
        .get_subject()
        .chars()
        .skip_while(|x| x.is_whitespace())
        .map(|x| x.to_uppercase().to_string() != x.to_string())
        .next()
        .unwrap_or(false)
}

pub fn lint(commit_message: &CommitMessage<'_>) -> Option<Problem> {
    if has_problem(commit_message) {
        Some(Problem::new(
            ERROR.into(),
            HELP_MESSAGE.into(),
            Code::SubjectNotCapitalized,
            commit_message,
            Some(vec![(
                "Not capitalised".to_string(),
                commit_message
                    .get_subject()
                    .chars()
                    .filter(|x| x.is_whitespace())
                    .count()
                    .saturating_sub(2),
                1_usize,
            )]),
            Some("https://git-scm.com/book/en/v2/Distributed-Git-Contributing-to-a-Project#_commit_guidelines".parse().unwrap()),
        ))
    } else {
        None
    }
}
