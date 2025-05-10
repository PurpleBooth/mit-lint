use std::{ops::Add, option::Option::None, sync::LazyLock};

use mit_commit::CommitMessage;
use regex::Regex;

use crate::model::{Code, Problem};

/// Canonical lint ID
pub const CONFIG: &str = "jira-issue-key-missing";
/// Advice on how to correct the problem
pub const HELP_MESSAGE: &str = "It's important to add the issue key because it allows us to link code back to the motivations \
for doing it, and in some cases provide an audit trail for compliance purposes.

You can fix this by adding a key like `JRA-123` to the commit message" ;
/// Description of the problem
pub const ERROR: &str = "Your commit message is missing a JIRA Issue Key";

static RE: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"(?mi)^\s*(\[?\b[A-Z]{2,}-\d+\b\]?)").unwrap());

pub fn lint(commit_message: &CommitMessage<'_>) -> Option<Problem> {
    let comment_char = commit_message.get_comment_char();
    let has_jira_key = String::from(commit_message.clone()).lines()
        .skip_while(|line| comment_char.map_or(false, |c| line.starts_with(c)))
        .any(|line| RE.is_match(line));
        
    if has_jira_key {
        None
    } else {
        let commit_text = String::from(commit_message.clone());
        let last_line_location = commit_text
            .trim_end()
            .rfind('\n')
            .unwrap_or_default()
            .add(1);
        Some(Problem::new(
            ERROR.into(),
            HELP_MESSAGE.into(),
            Code::JiraIssueKeyMissing,
            commit_message,
            Some(vec![(
                "No JIRA Issue Key".to_string(),
                last_line_location,
                commit_text.chars().count().saturating_sub(last_line_location + 1),
            )]),
            Some("https://support.atlassian.com/jira-software-cloud/docs/what-is-an-issue/#Workingwithissues-Projectkeys".to_string()),
        ))
    }
}
