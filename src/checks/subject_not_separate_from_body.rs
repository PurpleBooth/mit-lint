use std::option::Option::None;

use mit_commit::CommitMessage;

use crate::model::{Code, Problem};

/// Canonical lint ID
pub const CONFIG: &str = "subject-not-separated-from-body";
/// Description of the problem
pub const ERROR: &str =
    "Your commit message is missing a blank line between the subject and the body";
/// Advice on how to correct the problem
pub const HELP_MESSAGE: &str = "Most tools that render and parse commit messages, expect commit \
                            messages to be in the form of subject and body. This includes git \
                            itself in tools like git-format-patch. If you don't include this you \
                            may see strange behaviour from git and any related tools.\n\nTo fix \
                            this separate subject from body with a blank line";

fn has_problem(commit_message: &CommitMessage<'_>) -> bool {
    let subject: String = commit_message.get_subject().into();
    subject.lines().count() > 1
}

pub fn lint(commit_message: &CommitMessage<'_>) -> Option<Problem> {
    if has_problem(commit_message) {
        let commit_text = String::from(commit_message.clone());
        let mut lines = commit_text.lines();
        let first_line_length = lines.next().map(str::len).unwrap_or_default() + 1;
        let gutter_line_length = lines.next().map(str::len).unwrap_or_default();
        Some(Problem::new(
            ERROR.into(),
            HELP_MESSAGE.into(),
            Code::SubjectNotSeparateFromBody,
            commit_message,
            Some(vec![(
                "Missing blank line".to_string(),
                first_line_length,
                gutter_line_length,
            )]),
            Some("https://git-scm.com/book/en/v2/Distributed-Git-Contributing-to-a-Project#_commit_guidelines".parse().unwrap()),
        ))
    } else {
        None
    }
}
