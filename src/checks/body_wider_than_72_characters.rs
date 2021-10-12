use std::{ops::Add, option::Option::None};

use miette::SourceOffset;
use mit_commit::CommitMessage;

use crate::model::{Code, Problem};

/// Canonical lint ID
pub const CONFIG: &str = "body-wider-than-72-characters";

/// Advice on how to correct the problem
pub const HELP_MESSAGE: &str = "It's important to keep the body of the commit narrower than 72 \
                            characters because when you look at the git log, that's where it \
                            truncates the message. This means that people won't get the entirety \
                            of the information in your commit.\n\nYou can fix this by making the \
                            lines in your body no more than 72 characters";
/// Description of the problem
pub const ERROR: &str = "Your commit has a body wider than 72 characters";

fn has_problem(commit: &CommitMessage) -> bool {
    commit
        .get_body()
        .iter()
        .flat_map(|body| {
            String::from(body.clone())
                .lines()
                .map(String::from)
                .collect::<Vec<String>>()
        })
        .any(|line| line.chars().count() > 72)
}

const LIMIT: usize = 72;

pub fn lint(commit: &CommitMessage) -> Option<Problem> {
    if has_problem(commit) {
        let commit_text = String::from(commit.clone());

        Some(Problem::new(
            ERROR.into(),
            HELP_MESSAGE.into(),
            Code::BodyWiderThan72Characters,
            commit,
            Some(
                commit_text
                    .clone()
                    .lines()
                    .enumerate()
                    .filter(|(line_index, line)| line_index > &0 && line.len() > LIMIT)
                    .map(|(line_index, line)| {
                        (
                            "Too long".to_string(),
                            SourceOffset::from_location(
                                commit_text.clone(),
                                line_index.add(1),
                                LIMIT.add(2),
                            )
                            .offset(),
                            line.len() - (LIMIT),
                        )
                    })
                    .collect(),
            ),
            Some("https://git-scm.com/book/en/v2/Distributed-Git-Contributing-to-a-Project#_commit_guidelines".to_string()),
        ))
    } else {
        None
    }
}
