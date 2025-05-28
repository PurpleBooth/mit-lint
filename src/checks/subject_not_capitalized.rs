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
        .find(|x| !x.is_whitespace())
        .filter(|x| x.is_lowercase())
        .is_some()
}

pub fn lint(commit_message: &CommitMessage<'_>) -> Option<Problem> {
    fn label_position(commit_message: &CommitMessage) -> usize {
        commit_message
            .get_subject()
            .chars()
            .filter(|x| x.is_whitespace())
            .count()
            .saturating_sub(2)
    }

    Some(commit_message)
        .filter(|commit_message| has_problem(commit_message))
        .map(|commit_message: &CommitMessage| Problem::new(
            ERROR.into(),
            HELP_MESSAGE.into(),
            Code::SubjectNotCapitalized,
            commit_message,
            Some(vec![(
                "Not capitalised".to_string(),
                label_position(commit_message),
                1_usize,
            )]),
            Some("https://git-scm.com/book/en/v2/Distributed-Git-Contributing-to-a-Project#_commit_guidelines".parse().unwrap()),
        ))
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::option::Option::None;

    use crate::{Code, Problem};
    use miette::{GraphicalReportHandler, GraphicalTheme, Report};
    use mit_commit::CommitMessage;
    use quickcheck::TestResult;

    #[test]
    fn test_capitalized_subject_passes() {
        run_test("Subject Line", None);
    }

    #[test]
    fn test_lowercase_subject_fails() {
        run_test(
            "subject line
",
            Some(Problem::new(
                ERROR.into(),
                HELP_MESSAGE.into(),
                Code::SubjectNotCapitalized,
                &"subject line
"
                    .into(),
                Some(vec![("Not capitalised".to_string(), 0_usize, 1_usize)]),
                Some("https://git-scm.com/book/en/v2/Distributed-Git-Contributing-to-a-Project#_commit_guidelines".parse().unwrap()),
            )).as_ref(),
        );
    }

    #[test]
    fn test_leading_space_with_lowercase_subject_fails() {
        run_test(
            "  subject line",
            Some(Problem::new(
                ERROR.into(),
                HELP_MESSAGE.into(),
                Code::SubjectNotCapitalized,
                &CommitMessage::from("  subject line"),
                Some(vec![("Not capitalised".to_string(), 1_usize, 1_usize)]),
                Some("https://git-scm.com/book/en/v2/Distributed-Git-Contributing-to-a-Project#_commit_guidelines".to_string()),
            )).as_ref(),
        );
    }

    #[test]
    fn test_numeric_subject_passes() {
        run_test(
            "1234567
", None,
        );
    }

    #[test]
    fn test_error_formatting_matches_expected_output() {
        let message = "  an example commit\n\nexample";
        let problem = lint(&CommitMessage::from(message.to_string()));
        let actual = fmt_report(&Report::new(problem.unwrap()));
        let expected = "SubjectNotCapitalized (https://git-scm.com/book/en/v2/Distributed-Git-Contributing-to-a-Project#_commit_guidelines)

  x Your commit message is missing a capital letter
   ,-[1:3]
 1 |   an example commit
   :   |
   :   `-- Not capitalised
 2 | 
   `----
  help: The subject line is a title, and as such should be capitalised.
        
        You can fix this by capitalising the first character in the subject
"
            .to_string();
        assert_eq!(
            actual, expected,
            "Message {message:?} should have returned {expected:?}, found {actual:?}"
        );
    }

    #[allow(clippy::needless_pass_by_value)]
    #[quickcheck]
    fn test_lowercase_first_character_always_fails(commit_message_body: String) -> TestResult {
        match commit_message_body
            .chars()
            .take_while(|x| *x != '\n')
            .find(|x| !x.is_whitespace())
        {
            None => return TestResult::discard(),
            Some(char) => {
                if char.to_uppercase().to_string() == char.to_string() {
                    return TestResult::discard();
                }
            }
        }

        let message = CommitMessage::from(format!("{commit_message_body}\n# commit"));
        let result = lint(&message);
        let b = result.is_some();
        TestResult::from_bool(b)
    }

    #[allow(clippy::needless_pass_by_value)]
    #[quickcheck]
    fn test_uppercase_first_character_always_passes(commit_message_body: String) -> TestResult {
        if commit_message_body.starts_with('#') {
            return TestResult::discard();
        }

        match commit_message_body
            .chars()
            .take_while(|x| *x != '\n')
            .find(|x| !x.is_whitespace())
        {
            None => return TestResult::discard(),
            Some(char) => {
                if char.is_lowercase() {
                    return TestResult::discard();
                }
            }
        }

        let message = CommitMessage::from(format!("{commit_message_body}\n# commit"));
        let result = lint(&message);
        TestResult::from_bool(result.is_none())
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

    fn run_test(message: &str, expected: Option<&Problem>) {
        let actual = lint(&CommitMessage::from(message));
        assert_eq!(
            actual.as_ref(),
            expected,
            "Message {message:?} should have returned {expected:?}, found {actual:?}"
        );
    }
}
