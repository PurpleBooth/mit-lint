use std::option::Option::None;

use miette::{GraphicalReportHandler, GraphicalTheme, Report};
use mit_commit::CommitMessage;
use quickcheck::TestResult;
use strum::IntoEnumIterator;

use super::not_emoji_log::{lint, ERROR, HELP_MESSAGE};
use crate::{
    checks::not_emoji_log::Prefix,
    model::{Code, Problem},
};

#[test]
fn new() {
    run_lint(
        "\u{1f4e6} NEW: An example commit

This is an example commit
",
        &None,
    );
}

#[test]
fn improve() {
    run_lint(
        "\u{1f44c} IMPROVE: An example commit

This is an example commit
",
        &None,
    );
}

#[test]
fn fix() {
    run_lint(
        "\u{1f41b} FIX: An example commit

This is an example commit
",
        &None,
    );
}

#[test]
fn docs() {
    run_lint(
        "\u{1f4d6} DOC: An example commit

This is an example commit
",
        &None,
    );
}

#[test]
fn release() {
    run_lint(
        "\u{1f680} RELEASE: An example commit

This is an example commit
",
        &None,
    );
}

#[test]
fn test() {
    run_lint(
        "\u{1f916} TEST: An example commit

This is an example commit
",
        &None,
    );
}

#[test]
fn bc() {
    run_lint(
        "\u{203c}\u{fe0f} BREAKING: An example commit

This is an example commit
",
        &None,
    );
}

#[test]
fn no_gap() {
    let message = "\u{203c}\u{fe0f} BREAKING:An example commit

This is an example commit
";
    run_lint(
        message,
        &Some(Problem::new(
            ERROR.into(),
            HELP_MESSAGE.into(),
            Code::NotEmojiLog,
            &message.into(),
            Some(vec![("Not emoji log".to_string(), 0_usize, 33_usize)]),
            Some("https://github.com/ahmadawais/Emoji-Log".to_string()),
        )),
    );
}

#[test]
fn unknown_emoji() {
    let message = "\u{1f408} UNKNOWN: An example commit

This is an example commit
";
    run_lint(
        message,
        &Some(Problem::new(
            ERROR.into(),
            HELP_MESSAGE.into(),
            Code::NotEmojiLog,
            &message.into(),
            Some(vec![("Not emoji log".to_string(), 0_usize, 31_usize)]),
            Some("https://github.com/ahmadawais/Emoji-Log".to_string()),
        )),
    );
}

#[test]
fn not_emoji() {
    let message = "An example commit

This is an example commit
";
    run_lint(
        message,
        &Some(Problem::new(
            ERROR.into(),
            HELP_MESSAGE.into(),
            Code::NotEmojiLog,
            &message.into(),
            Some(vec![("Not emoji log".to_string(), 0_usize, 17_usize)]),
            Some("https://github.com/ahmadawais/Emoji-Log".to_string()),
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
    let expected = "NotEmojiLog (https://github.com/ahmadawais/Emoji-Log)

  x Your commit message isn't in emoji log style
   ,-[1:1]
 1 | An example commit
   : ^^^^^^^^|^^^^^^^^
   :         `-- Not emoji log
 2 | 
   `----
  help: It's important to follow the emoji log style when creating your
        commit message. By using this style we can automatically generate
        changelogs.
        
        You can fix it using one of the prefixes:
        
        
        \u{1f4e6} NEW:
        \u{1f44c} IMPROVE:
        \u{1f41b} FIX:
        \u{1f4d6} DOC:
        \u{1f680} RELEASE:
        \u{1f916} TEST:
        \u{203c}\u{fe0f} BREAKING:
"
    .to_string();
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

fn run_lint(message: &str, expected: &Option<Problem>) {
    let actual = &lint(&CommitMessage::from(message));
    assert_eq!(
        actual, expected,
        "Message {message:?} should have returned {expected:?}, found {actual:?}"
    );
}

#[test]
fn emoji_log_prefixes_new() {
    let input: Prefix = Prefix::New;

    assert_eq!(String::from(input), "\u{1f4e6} NEW: ".to_string());
}

#[test]
fn emoji_log_prefixes_improve() {
    let input: Prefix = Prefix::Improve;

    assert_eq!(String::from(input), "\u{1f44c} IMPROVE: ".to_string());
}

#[test]
fn emoji_log_prefixes_fix() {
    let input: Prefix = Prefix::Fix;

    assert_eq!(String::from(input), "\u{1f41b} FIX: ".to_string());
}

#[test]
fn emoji_log_prefixes_docs() {
    let input: Prefix = Prefix::Doc;

    assert_eq!(String::from(input), "\u{1f4d6} DOC: ".to_string());
}

#[test]
fn emoji_log_prefixes_release() {
    let input: Prefix = Prefix::Release;

    assert_eq!(String::from(input), "\u{1f680} RELEASE: ".to_string());
}

#[test]
fn emoji_log_prefixes_test() {
    let input: Prefix = Prefix::Test;

    assert_eq!(String::from(input), "\u{1f916} TEST: ".to_string());
}

#[test]
fn emoji_log_prefixes_breaking() {
    let input: Prefix = Prefix::Breaking;

    assert_eq!(
        String::from(input),
        "\u{203c}\u{fe0f} BREAKING: ".to_string()
    );
}

#[allow(clippy::needless_pass_by_value)]
#[quickcheck]
fn get_prefixes(prefix: Prefix) -> bool {
    Prefix::iter().any(|x| x == prefix)
}

#[allow(clippy::needless_pass_by_value)]
#[quickcheck]
fn success_check(prefix: Prefix, subject: String, body: Option<String>) -> TestResult {
    if subject.contains('\n') {
        return TestResult::discard();
    }

    let message = CommitMessage::from(format!(
        "{}{}{}\n# Comment",
        String::from(prefix),
        subject,
        body.map(|x| format!("\n\n{x}")).unwrap_or_default()
    ));
    let result = lint(&message);
    TestResult::from_bool(result.is_none())
}

#[allow(clippy::needless_pass_by_value)]
#[quickcheck]
fn fail_check(commit: String) -> TestResult {
    if Prefix::iter()
        .map(String::from)
        .any(|x| commit.starts_with(&x))
    {
        return TestResult::discard();
    }

    let message = CommitMessage::from(commit);
    let result = lint(&message);
    TestResult::from_bool(result.is_some())
}
