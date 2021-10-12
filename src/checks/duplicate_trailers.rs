use std::{collections::BTreeMap, ops::Add, option::Option::None};

use mit_commit::{CommitMessage, Trailer};

use crate::model::{Code, Problem};

/// Canonical lint ID
pub const CONFIG: &str = "duplicated-trailers";

const TRAILERS_TO_CHECK_FOR_DUPLICATES: [&str; 3] =
    ["Signed-off-by", "Co-authored-by", "Relates-to"];
const FIELD_SINGULAR: &str = "field";
/// Description of the problem
pub const ERROR: &str = "Your commit message has duplicated trailers";

const FIELD_PLURAL: &str = "fields";

fn get_duplicated_trailers(commit_message: &CommitMessage) -> Vec<String> {
    commit_message
        .get_trailers()
        .iter()
        .fold(
            BTreeMap::new(),
            |acc: BTreeMap<&Trailer, usize>, trailer| {
                let mut next = acc.clone();
                match acc.get(trailer) {
                    Some(count) => next.insert(trailer, count.add(1)),
                    None => next.insert(trailer, 1),
                };

                next
            },
        )
        .into_iter()
        .filter_map(|(trailer, usize)| {
            let key: &str = &trailer.get_key();

            if usize > 1 && TRAILERS_TO_CHECK_FOR_DUPLICATES.contains(&key) {
                Some(trailer.get_key())
            } else {
                None
            }
        })
        .collect::<Vec<_>>()
}

pub fn lint(commit: &CommitMessage) -> Option<Problem> {
    let duplicated_trailers = get_duplicated_trailers(commit);
    if duplicated_trailers.is_empty() {
        None
    } else {
        let commit_text = String::from(commit.clone());
        let warning = warning(&duplicated_trailers);
        Some(Problem::new(
            ERROR.into(),
            warning,
            Code::DuplicatedTrailers,
            commit,
            Some(
                duplicated_trailers
                    .into_iter()
                    .flat_map(|trailer| {
                        commit_text
                            .match_indices(&trailer)
                            .skip(1)
                            .map(|x| {
                                (
                                    format!("Duplicated `{}`", trailer),
                                    x.0,
                                    commit_text
                                        .chars()
                                        .skip(x.0)
                                        .take_while(|x| x != &'\n')
                                        .count(),
                                )
                            })
                            .collect::<Vec<_>>()
                    })
                    .collect(),
            ),
            Some("https://git-scm.com/docs/githooks#_commit_msg".to_string()),
        ))
    }
}

fn warning(duplicated_trailers: &[String]) -> String {
    let warning = format!(
        "These are normally added accidentally when you're rebasing or amending to a commit, \
         sometimes in the text editor, but often by git hooks.\n\nYou can fix this by deleting \
         the duplicated \"{}\" {}",
        duplicated_trailers.join("\", \""),
        if duplicated_trailers.len() > 1 {
            FIELD_PLURAL
        } else {
            FIELD_SINGULAR
        }
    );
    warning
}
