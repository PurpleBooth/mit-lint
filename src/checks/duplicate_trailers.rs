use std::{collections::BTreeMap, option::Option::None};

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

fn get_duplicated_trailers(commit_message: &CommitMessage<'_>) -> Vec<String> {
    commit_message
        .get_trailers()
        .iter()
        .fold(
            BTreeMap::new(),
            |mut acc: BTreeMap<&Trailer<'_>, usize>, trailer| {
                let count = acc.get(trailer).map_or(1, |c| c + 1);
                acc.insert(trailer, count);
                acc
            },
        )
        .into_iter()
        .filter_map(|(trailer, count)| {
            let key = trailer.get_key();

            if count > 1 && TRAILERS_TO_CHECK_FOR_DUPLICATES.contains(&key.as_str()) {
                Some(key)
            } else {
                None
            }
        })
        .collect::<Vec<_>>()
}

/// Checks if the commit message contains duplicated trailers
///
/// # Arguments
///
/// * `commit` - The commit message to check
///
/// # Returns
///
/// * `Some(Problem)` - If the commit message contains duplicated trailers
/// * `None` - If the commit message does not contain duplicated trailers
///
/// # Examples
///
/// ```rust
/// use mit_commit::CommitMessage;
/// use mit_lint::Lint::DuplicatedTrailers;
///
/// // This should pass
/// let passing = CommitMessage::from("Subject\n\nBody\n\nSigned-off-by: Someone <someone@example.com>");
/// assert!(DuplicatedTrailers.lint(&passing).is_none());
///
/// // This should fail
/// let failing = CommitMessage::from(
///     "Subject\n\nBody\n\nSigned-off-by: Someone <someone@example.com>\nSigned-off-by: Someone <someone@example.com>"
/// );
/// assert!(DuplicatedTrailers.lint(&failing).is_some());
/// ```
pub fn lint(commit: &CommitMessage<'_>) -> Option<Problem> {
    let duplicated_trailers = get_duplicated_trailers(commit);
    if duplicated_trailers.is_empty() {
        None
    } else {
        // We need to clone here as String::from works with CommitMessage but not &CommitMessage
        let commit_message = String::from(commit.clone());
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
                        commit_message
                            .match_indices(&trailer)
                            .skip(1)
                            .map(|x| {
                                (
                                    format!("Duplicated `{trailer}`"),
                                    x.0,
                                    commit_message
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
    format!(
        "These are normally added accidentally when you're rebasing or amending to a commit, \
         sometimes in the text editor, but often by git hooks.\n\nYou can fix this by deleting \
         the duplicated \"{}\" {}",
        duplicated_trailers.join("\", \""),
        if duplicated_trailers.len() > 1 {
            FIELD_PLURAL
        } else {
            FIELD_SINGULAR
        }
    )
}
