#![allow(clippy::wildcard_imports)]

use std::option::Option::None;

use miette::{GraphicalReportHandler, GraphicalTheme, Report};
use mit_commit::CommitMessage;
use quickcheck::TestResult;

use super::missing_jira_issue_key::{lint, ERROR, HELP_MESSAGE};
use crate::model::{Code, Problem};

#[test]
fn ignores_jira_keys_in_comments() {
    test_has_missing_jira_issue_key(
        "An example commit\n\n# JRA-123 in comment",
        Some(Problem::new(
            ERROR.into(),
            HELP_MESSAGE.into(),
            Code::JiraIssueKeyMissing,
            &"An example commit\n\n# JRA-123 in comment".into(),
            Some(vec![("No JIRA Issue Key".to_string(), 19, 19)]),
            Some("https://support.atlassian.com/jira-software-cloud/docs/what-is-an-issue/#Workingwithissues-Projectkeys".parse().unwrap()),
        )).as_ref(),
    );
}

#[test]
fn id_present() {
    test_has_missing_jira_issue_key(
        "JRA-123 An example commit

This is an example commit
",
        None,
    );
    test_has_missing_jira_issue_key(
        "An example commit

This is an JRA-123 example commit
",
        None,
    );
    test_has_missing_jira_issue_key(
        "An example commit

JRA-123

This is an example commit
",
        None,
    );
    test_has_missing_jira_issue_key(
        "An example commit

This is an example commit

JRA-123
",
        None,
    );
    test_has_missing_jira_issue_key(
        "An example commit

This is an example commit

JR-123
",
        None,
    );
    test_has_missing_jira_issue_key(
        "An example commit

This is an example commit

Relates-to: [JRA-123]
",
        None,
    );
    test_has_missing_jira_issue_key(
        "[JRA-123] An example commit

This is an example commit
",
        None,
    );
    test_has_missing_jira_issue_key(
        "An example commit

This is an [JRA-123] example commit
",
        None,
    );
}

#[test]
fn id_missing() {
    let message_1 = "An example commit

This is an example commit
";
    test_has_missing_jira_issue_key(
        message_1,
        Some(Problem::new(
            ERROR.into(),
            HELP_MESSAGE.into(),
            Code::JiraIssueKeyMissing,
            &message_1.into(),
            Some(vec![("No JIRA Issue Key".to_string(), 19_usize, 25_usize)]),
            Some("https://support.atlassian.com/jira-software-cloud/docs/what-is-an-issue/#Workingwithissues-Projectkeys".parse().unwrap()),
        )).as_ref(),
    );
    let message_2 = "An example commit

This is an example commit

A-123
";
    test_has_missing_jira_issue_key(
        message_2,
        Some(Problem::new(
            ERROR.into(),
            HELP_MESSAGE.into(),
            Code::JiraIssueKeyMissing,
            &message_2.into(),
            Some(vec![("No JIRA Issue Key".to_string(), 46_usize, 5_usize)]),
            Some("https://support.atlassian.com/jira-software-cloud/docs/what-is-an-issue/#Workingwithissues-Projectkeys".parse().unwrap()),
        )).as_ref(),
    );
    let message_3 = "An example commit

This is an example commit

JRA-
";
    test_has_missing_jira_issue_key(
        message_3,
        Some(Problem::new(
            ERROR.into(),
            HELP_MESSAGE.into(),
            Code::JiraIssueKeyMissing,
            &message_3.into(),
            Some(vec![("No JIRA Issue Key".to_string(), 46_usize, 4_usize)]),
            Some("https://support.atlassian.com/jira-software-cloud/docs/what-is-an-issue/#Workingwithissues-Projectkeys".parse().unwrap()),
        )).as_ref(),
    );
}

#[test]
fn formatting() {
    let message = "An example commit

This is an example commit
";
    let problem = lint(&CommitMessage::from(message.to_string()));
    let actual = fmt_report(&Report::new(problem.unwrap()));
    let expected = "JiraIssueKeyMissing (https://support.atlassian.com/jira-software-cloud/docs/what-is-an-issue/#Workingwithissues-Projectkeys)

  x Your commit message is missing a JIRA Issue Key
   ,-[3:1]
 2 | 
 3 | This is an example commit
   : ^^^^^^^^^^^^|^^^^^^^^^^^^
   :             `-- No JIRA Issue Key
   `----
  help: It's important to add the issue key because it allows us to link code
        back to the motivations for doing it, and in some cases provide an
        audit trail for compliance purposes.
        
        You can fix this by adding a key like `JRA-123` to the commit message
" .to_string();
    assert_eq!(
        actual, expected,
        "Message {message:?} should have returned {expected:?}, found {actual:?}"
    );
}

fn fmt_report(diag: &Report) -> String {
    let mut out = String::new();
    GraphicalReportHandler::new_themed(GraphicalTheme::none())
        .with_width(80)
        .with_links(false)
        .render_report(&mut out, diag.as_ref())
        .unwrap();
    out
}

fn test_has_missing_jira_issue_key(message: &str, expected: Option<&Problem>) {
    let actual = lint(&CommitMessage::from(message));
    assert_eq!(
        actual.as_ref(),
        expected,
        "Message {message:?} should have returned {expected:?}, found {actual:?}"
    );
}

#[allow(clippy::needless_pass_by_value)]
#[quickcheck]
fn fail_check(commit: String) -> TestResult {
    let message = CommitMessage::from(commit);
    let result = lint(&message);
    TestResult::from_bool(result.is_some())
}

#[allow(clippy::needless_pass_by_value)]
#[quickcheck]
fn success_check(
    before: Option<String>,
    characters: String,
    numbers: usize,
    after: Option<String>,
) -> TestResult {
    if characters.chars().count() < 2
        || characters
            .chars()
            .any(|x| !x.is_ascii_alphabetic() || !x.is_uppercase())
    {
        return TestResult::discard();
    }

    let message = CommitMessage::from(format!(
        "{}{}-{}{}\n# comment",
        before.map(|x| format!("{x} ")).unwrap_or_default(),
        characters,
        numbers,
        after.map(|x| format!(" {x} ")).unwrap_or_default(),
    ));
    let result = lint(&message);
    TestResult::from_bool(result.is_none())
}
