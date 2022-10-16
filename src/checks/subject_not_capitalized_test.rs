use std::option::Option::None;

use miette::{GraphicalReportHandler, GraphicalTheme, Report};
use mit_commit::CommitMessage;
use quickcheck::TestResult;

use super::subject_not_capitalized::{lint, ERROR, HELP_MESSAGE};
use crate::{Code, Problem};

#[test]
fn capitalised() {
    run_test("Subject Line", &None);
}

#[test]
fn lower_case() {
    run_test(
        "subject line
",
        &Some(Problem::new(
            ERROR.into(),
            HELP_MESSAGE.into(),
            Code::SubjectNotCapitalized,
            &"subject line
"
                .into(),
            Some(vec![("Not capitalised".to_string(), 0_usize, 1_usize)]),
            Some("https://git-scm.com/book/en/v2/Distributed-Git-Contributing-to-a-Project#_commit_guidelines".parse().unwrap()),
        )),
    );
}

#[test]
fn space_first() {
    run_test(
        "  subject line",
        &Some(Problem::new(
            ERROR.into(),
            HELP_MESSAGE.into(),
            Code::SubjectNotCapitalized,
            &CommitMessage::from("  subject line"),
            Some(vec![("Not capitalised".to_string(), 1_usize, 1_usize)]),
            Some("https://git-scm.com/book/en/v2/Distributed-Git-Contributing-to-a-Project#_commit_guidelines".to_string()),
        )),
    );
}

#[test]
fn numbers_are_fine() {
    run_test(
        "1234567
", &None,
    );
}

#[test]
fn formatting() {
    let message = "  an example commit\n\nexample";
    let problem = lint(&CommitMessage::from(message.to_string()));
    let actual = fmt_report(&Report::new(problem.unwrap()));
    let expected = "SubjectNotCapitalized (https://git-scm.com/book/en/v2/Distributed-Git-Contributing-to-a-Project#_commit_guidelines)

  x Your commit message is missing a capital letter
   ,-[1:1]
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
        "Message {:?} should have returned {:?}, found {:?}",
        message, expected, actual
    );
}

#[allow(clippy::needless_pass_by_value)]
#[quickcheck]
fn failure_check(commit_message_body: String) -> TestResult {
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
fn success_check(commit_message_body: String) -> TestResult {
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

fn run_test(message: &str, expected: &Option<Problem>) {
    let actual = &lint(&CommitMessage::from(message));
    assert_eq!(
        actual, expected,
        "Message {:?} should have returned {:?}, found {:?}",
        message, expected, actual
    );
}
