use std::{ops::Add, option::Option::None};

use mit_commit::CommitMessage;

use crate::model::{Code, Problem};

/// Canonical lint ID
pub(crate) const CONFIG: &str = "jira-issue-key-missing";
/// Advice on how to correct the problem
const HELP_MESSAGE: &str = "It's important to add the issue key because it allows us to link code back to the motivations \
for doing it, and in some cases provide an audit trail for compliance purposes.

You can fix this by adding a key like `JRA-123` to the commit message" ;
/// Description of the problem
const ERROR: &str = "Your commit message is missing a JIRA Issue Key";

lazy_static! {
    static ref RE: regex::Regex = regex::Regex::new(r"(?m)(^| )[A-Z]{2,}-[0-9]+( |$)").unwrap();
}

pub(crate) fn lint(commit_message: &CommitMessage) -> Option<Problem> {
    if commit_message.matches_pattern(&*RE) {
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
                commit_text.len().saturating_sub(last_line_location),
            )]),
            None,
        ))
    }
}

#[cfg(test)]
mod tests_has_missing_jira_issue_key {
    #![allow(clippy::wildcard_imports)]

    use std::option::Option::None;

    use miette::{GraphicalReportHandler, GraphicalTheme, Report};

    use super::*;
    use crate::model::{Code, Problem};

    #[test]
    fn id_present() {
        test_has_missing_jira_issue_key(
            "JRA-123 An example commit

This is an example commit
",
            &None,
        );
        test_has_missing_jira_issue_key(
            "An example commit

This is an JRA-123 example commit
",
            &None,
        );
        test_has_missing_jira_issue_key(
            "An example commit

JRA-123

This is an example commit
",
            &None,
        );
        test_has_missing_jira_issue_key(
            "An example commit

This is an example commit

JRA-123
",
            &None,
        );
        test_has_missing_jira_issue_key(
            "An example commit

This is an example commit

JR-123
",
            &None,
        );
    }

    #[test]
    fn id_missing() {
        let message_1 = "An example commit

This is an example commit
";
        test_has_missing_jira_issue_key(
            message_1,
            &Some(Problem::new(
                ERROR.into(),
                HELP_MESSAGE.into(),
                Code::JiraIssueKeyMissing,
                &message_1.into(),
                Some(vec![("No JIRA Issue Key".to_string(), 19_usize, 26_usize)]),
                None,
            )),
        );
        let message_2 = "An example commit

This is an example commit

A-123
";
        test_has_missing_jira_issue_key(
            message_2,
            &Some(Problem::new(
                ERROR.into(),
                HELP_MESSAGE.into(),
                Code::JiraIssueKeyMissing,
                &message_2.into(),
                Some(vec![("No JIRA Issue Key".to_string(), 46_usize, 6_usize)]),
                None,
            )),
        );
        let message_3 = "An example commit

This is an example commit

JRA-
";
        test_has_missing_jira_issue_key(
            message_3,
            &Some(Problem::new(
                ERROR.into(),
                HELP_MESSAGE.into(),
                Code::JiraIssueKeyMissing,
                &message_3.into(),
                Some(vec![("No JIRA Issue Key".to_string(), 46_usize, 5_usize)]),
                None,
            )),
        );
    }

    #[test]
    fn formatting() {
        let message = "An example commit

This is an example commit
";
        let problem = lint(&CommitMessage::from(message.to_string()));
        let actual = fmt_report(&Report::new(problem.unwrap()));
        let expected = "JiraIssueKeyMissing

  \u{d7} Your commit message is missing a JIRA Issue Key
   \u{256d}\u{2500}[2:1]
 2 \u{2502} 
 3 \u{2502} This is an example commit
   \u{b7} \u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{252c}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}
   \u{b7}              \u{2570}\u{2500}\u{2500} No JIRA Issue Key
   \u{2570}\u{2500}\u{2500}\u{2500}\u{2500}
  help: It's important to add the issue key because it allows us to link
        code back to the motivations for doing it, and in some cases provide
        an audit trail for compliance purposes.
        
        You can fix this by adding a key like `JRA-123` to the commit
        message
" .to_string();
        assert_eq!(
            actual, expected,
            "Message {:?} should have returned {:?}, found {:?}",
            message, expected, actual
        );
    }

    fn fmt_report(diag: &Report) -> String {
        let mut out = String::new();
        GraphicalReportHandler::new_themed(GraphicalTheme::unicode_nocolor())
            .with_width(80)
            .render_report(&mut out, diag.as_ref())
            .unwrap();
        out
    }
    fn test_has_missing_jira_issue_key(message: &str, expected: &Option<Problem>) {
        let actual = &lint(&CommitMessage::from(message));
        assert_eq!(
            actual, expected,
            "Message {:?} should have returned {:?}, found {:?}",
            message, expected, actual
        );
    }
}
