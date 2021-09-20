use mit_commit::CommitMessage;

use crate::model::{Code, Problem};

// Canonical lint ID
pub(crate) const CONFIG: &str = "jira-issue-key-missing";
// Advice on how to correct the problem
const HELP_MESSAGE: &str = indoc::indoc!(
    "
    It's important to add the issue key because it allows us to link code back to the motivations \
     for doing it, and in some cases provide an audit trail for compliance purposes.

    You can fix this by adding a key like `JRA-123` to the commit message"
);
// Description of the problem
const ERROR: &str = "Your commit message is missing a JIRA Issue Key";

lazy_static! {
    static ref RE: regex::Regex = regex::Regex::new(r"(?m)(^| )[A-Z]{2,}-[0-9]+( |$)").unwrap();
}

pub(crate) fn lint(commit_message: &CommitMessage) -> Option<Problem> {
    if commit_message.matches_pattern(&*RE) {
        None
    } else {
        Some(Problem::new(
            ERROR.into(),
            HELP_MESSAGE.into(),
            Code::JiraIssueKeyMissing,
        ))
    }
}

#[cfg(test)]
mod tests_has_missing_jira_issue_key {
    #![allow(clippy::wildcard_imports)]

    use indoc::indoc;

    use crate::model::{Code, Problem};

    use super::*;

    #[test]
    fn id_present() {
        test_has_missing_jira_issue_key(
            indoc!(
                "
                JRA-123 An example commit

                This is an example commit
                "
            ),
            &None,
        );
        test_has_missing_jira_issue_key(
            indoc!(
                "
                An example commit

                This is an JRA-123 example commit
                "
            ),
            &None,
        );
        test_has_missing_jira_issue_key(
            indoc!(
                "
                An example commit

                JRA-123

                This is an example commit
                "
            ),
            &None,
        );
        test_has_missing_jira_issue_key(
            indoc!(
                "
                An example commit

                This is an example commit

                JRA-123
                "
            ),
            &None,
        );
        test_has_missing_jira_issue_key(
            indoc!(
                "
                An example commit

                This is an example commit

                JR-123
                "
            ),
            &None,
        );
    }

    #[test]
    fn id_missing() {
        test_has_missing_jira_issue_key(
            indoc!(
                "
                An example commit

                This is an example commit
                "
            ),
            &Some(Problem::new(
                ERROR.into(),
                HELP_MESSAGE.into(),
                Code::JiraIssueKeyMissing,
            )),
        );
        test_has_missing_jira_issue_key(
            indoc!(
                "
                An example commit

                This is an example commit

                A-123
                "
            ),
            &Some(Problem::new(
                ERROR.into(),
                HELP_MESSAGE.into(),
                Code::JiraIssueKeyMissing,
            )),
        );
        test_has_missing_jira_issue_key(
            indoc!(
                "
                An example commit

                This is an example commit

                JRA-
                "
            ),
            &Some(Problem::new(
                ERROR.into(),
                HELP_MESSAGE.into(),
                Code::JiraIssueKeyMissing,
            )),
        );
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
