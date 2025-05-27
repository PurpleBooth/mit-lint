use std::option::Option::None;

use miette::{ByteOffset, SourceOffset};
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

fn has_problem(commit: &CommitMessage<'_>) -> bool {
    commit
        .get_body()
        .to_string()
        .lines()
        .any(|line| line.chars().count() > LIMIT)
}

const LIMIT: usize = 72;

/// Checks if the commit message body has lines wider than 72 characters
///
/// # Arguments
///
/// * `commit` - The commit message to check
///
/// # Returns
///
/// * `Some(Problem)` - If the commit message body has lines wider than 72 characters
/// * `None` - If the commit message body has no lines wider than 72 characters
///
/// # Examples
///
/// ```rust
/// use mit_commit::CommitMessage;
/// use mit_lint::Lint;
///
/// // This should pass
/// let passing = CommitMessage::from("Subject\n\nBody that is less than 72 chars wide");
/// assert!(Lint::BodyWiderThan72Characters.lint(&passing).is_none());
///
/// // This should fail
/// let failing = CommitMessage::from(
///     "Subject\n\nThis line is way too long and exceeds the 72 character limit by quite a bit actually"
/// );
/// assert!(Lint::BodyWiderThan72Characters.lint(&failing).is_some());
/// ```
pub fn lint(commit: &CommitMessage<'_>) -> Option<Problem> {
    if !has_problem(commit) {
        return None;
    }
    let comment_char = commit.get_comment_char().map(|x| format!("{x} "));
    let commit_text: String = commit.clone().into();
    let scissors_start_line = commit_text.lines().count()
        - commit
            .get_scissors()
            .map(|s| String::from(s).lines().count())
            .unwrap_or_default();
    let labels = commit_text
        .lines()
        .enumerate()
        .filter(|(line_index, line)| {
            *line_index <= scissors_start_line && is_line_over_limit(line, comment_char.as_deref())
        })
        .map(|(line_index, line)| label_line_over_limit(&commit_text, line_index, line))
        .collect();

    Some(Problem::new(
            ERROR.into(),
            HELP_MESSAGE.into(),
            Code::BodyWiderThan72Characters,
            commit,
            Some(
                labels,
            ),
            Some("https://git-scm.com/book/en/v2/Distributed-Git-Contributing-to-a-Project#_commit_guidelines".to_string()),
        ))
}

fn is_line_over_limit(line: &str, comment_char: Option<&str>) -> bool {
    let is_comment = comment_char.is_some_and(|cc| line.starts_with(cc));
    !is_comment && line.len() > LIMIT
}

fn label_line_over_limit(
    commit_text: &str,
    line_index: usize,
    line: &str,
) -> (String, ByteOffset, usize) {
    let char_count = line.chars().count();
    // Calculate character-based position accounting for multi-byte characters
    let char_offset = line.chars().take(LIMIT).map(char::len_utf8).sum::<usize>();

    (
        "Too long".to_string(),
        SourceOffset::from_location(commit_text, line_index + 1, char_offset + 1).offset(),
        char_count.saturating_sub(LIMIT),
    )
}
