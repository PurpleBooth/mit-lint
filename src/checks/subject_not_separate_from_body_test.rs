use std::option::Option::None;

use miette::{GraphicalReportHandler, GraphicalTheme, Report};
use mit_commit::CommitMessage;
use quickcheck::TestResult;

use crate::{
    checks::subject_not_separate_from_body::{lint, ERROR, HELP_MESSAGE},
    model::{Code, Problem},
};

#[test]
fn with_gutter() {
    test_subject_not_separate_from_body(
        "An example commit

This is an example commit
",
        None,
    );
    test_subject_not_separate_from_body(
        "Another example

Disabling this specific lint - Co-authored

Co-authored-by: Someone Else <someone@example.com>
Co-authored-by: Someone Else <someone@example.com>
",
        None,
    );
}

#[test]
fn single_line_with_trailing_newline() {
    test_subject_not_separate_from_body("An example commit\n", None);
}

#[test]
fn single_line_with_long_comments() {
    test_subject_not_separate_from_body(
        "Remove duplicated function
# Short (50 chars or less) summary of changes
#
# More detailed explanatory text, if necessary.  Wrap it to
# about 72 characters or so.  In some contexts, the first
# line is treated as the subject of an email and the rest of
# the text as the body.  The blank line separating the
# summary from the body is critical (unless you omit the body
# entirely); tools like rebase can get confused if you run
# the two together.
#
# Further paragraphs come after blank lines.
#
#   - Bullet points are okay, too
#
#   - Typically a hyphen or asterisk is used for the bullet,
#     preceded by a single space, with blank lines in
#     between, but conventions vary here

# Bitte geben Sie eine Commit-Beschreibung f\u{00FC}r Ihre \u{00C4}nderungen ein. \
     Zeilen,
# die mit '#' beginnen, werden ignoriert, und eine leere Beschreibung
# bricht den Commit ab.
#
# Auf Branch character-limit
# Zum Commit vorgemerkte \u{00C4}nderungen:
#	ge\u{00E4}ndert:       \
     mit-commit-message-lints/src/lints/model/missing_pivotal_tracker_id.rs
#
# ------------------------ >8 ------------------------
# \u{00C4}ndern oder entfernen Sie nicht die obige Zeile.
# Alles unterhalb von ihr wird ignoriert.
diff --git a/mit-commit-message-lints/src/lints/model/missing_pivotal_tracker_id.rs \
     b/mit-commit-message-lints/src/lints/model/missing_pivotal_tracker_id.rs
index 5a83784..ebaee48 100644
--- a/mit-commit-message-lints/src/lints/model/missing_pivotal_tracker_id.rs
+++ b/mit-commit-message-lints/src/lints/model/missing_pivotal_tracker_id.rs
-fn has_missing_pivotal_tracker_id(commit_message: &CommitMessage) -> bool {
-    has_no_pivotal_tracker_id(commit_message)
-}
-
 fn has_no_pivotal_tracker_id(text: &CommitMessage) -> bool {
     let re = Regex::new(REGEX_PIVOTAL_TRACKER_ID).unwrap();
     !text.matches_pattern(&re)
 }

 pub(crate) fn lint(commit_message: &CommitMessage) -> Option<Problem> {
-    if has_missing_pivotal_tracker_id(commit_message) {
+    if has_no_pivotal_tracker_id(commit_message) {
         Some(Problem::new(
             PIVOTAL_TRACKER_HELP.into(),
             Code::PivotalTrackerIdMissing,


",
        None,
    );
}

#[test]
fn single_line() {
    test_subject_not_separate_from_body("An example commit", None);
}

#[test]
fn gutter_missing() {
    test_subject_not_separate_from_body(
        "An example commit
This is an example commit
",
        Some(Problem::new(
            ERROR.into(),
            HELP_MESSAGE.into(),
            Code::SubjectNotSeparateFromBody,
            &"An example commit
This is an example commit
"
            .into(),
            Some(vec![("Missing blank line".to_string(), 18_usize, 25_usize)]),
            Some("https://git-scm.com/book/en/v2/Distributed-Git-Contributing-to-a-Project#_commit_guidelines".parse().unwrap()),
        )).as_ref(),
        );
    test_subject_not_separate_from_body(
        "An example commit
This is an example commit
It has more lines
It has even more lines
",
        Some(Problem::new(
            ERROR.into(),
            HELP_MESSAGE.into(),
            Code::SubjectNotSeparateFromBody,
            &"An example commit
This is an example commit
It has more lines
It has even more lines
"
            .into(),
            Some(vec![("Missing blank line".to_string(), 18_usize, 25_usize)]),
            Some("https://git-scm.com/book/en/v2/Distributed-Git-Contributing-to-a-Project#_commit_guidelines".to_string()),
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
    let expected = "SubjectNotSeparateFromBody (https://git-scm.com/book/en/v2/Distributed-Git-Contributing-to-a-Project#_commit_guidelines)

  x Your commit message is missing a blank line between the subject and the
  | body
   ,-[2:1]
 1 | An example commit
 2 | This is an example commit
   : ^^^^^^^^^^^^|^^^^^^^^^^^^
   :             `-- Missing blank line
   `----
  help: Most tools that render and parse commit messages, expect commit
        messages to be in the form of subject and body. This includes git
        itself in tools like git-format-patch. If you don't include this you
        may see strange behaviour from git and any related tools.
        
        To fix this separate subject from body with a blank line
"        .to_string();
    assert_eq!(
        actual, expected,
        "Message {message:?} should have returned {expected:?}, found {actual:?}"
    );
}

#[allow(clippy::needless_pass_by_value)]
#[quickcheck]
fn failure_check(subject: String, commit_message_body: String) -> TestResult {
    if subject.is_empty() || subject.lines().any(str::is_empty) || subject.lines().count() < 2 {
        return TestResult::discard();
    }

    let message = CommitMessage::default()
        .with_subject(subject.into())
        .with_body_contents(&commit_message_body);

    let result = lint(&message);
    TestResult::from_bool(result.is_some())
}

#[allow(clippy::needless_pass_by_value)]
#[quickcheck]
fn success_check(subject: String, commit_message_body: Option<String>) -> TestResult {
    if subject.contains('\n') {
        return TestResult::discard();
    }
    if subject.chars().position(|x| x == '#').unwrap_or(72) < 71 {
        return TestResult::discard();
    }
    let text = format!(
        "{}{}",
        subject,
        commit_message_body
            .map(|x| format!("\n\n{x}"))
            .unwrap_or_default()
    );

    let message = CommitMessage::from(format!("{text}\n# commit"));
    let result = lint(&message);
    let actual = result.is_none();
    TestResult::from_bool(actual)
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

fn test_subject_not_separate_from_body(message: &str, expected: Option<&Problem>) {
    let actual = lint(&CommitMessage::from(message));
    assert_eq!(
        actual.as_ref(),
        expected,
        "Message {message:?} should have returned {expected:?}, found {actual:?}"
    );
}
