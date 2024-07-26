#![allow(clippy::wildcard_imports)]

use std::option::Option::None;

use miette::{GraphicalReportHandler, GraphicalTheme, Report};
use mit_commit::CommitMessage;
use quickcheck::TestResult;

use super::body_wider_than_72_characters::{lint, ERROR, HELP_MESSAGE};
use crate::{model::Code, Problem};

#[test]
fn narrower_than_72_characters() {
    test_body_wider_than_72_characters(&format!("Subject\n\n{}", "x".repeat(72)), &None);
}

#[test]
fn no_body() {
    test_body_wider_than_72_characters("Subject", &None);
}

#[test]
fn body_ok_but_comments_longer_than_72() {
    let message = "Remove duplicated function

The function got skipped in thee previous round of refactoring
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
#       ge\u{00E4}ndert:       \
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


";
    test_body_wider_than_72_characters(&format!("{}\n\n{message}", "x".repeat(72)), &None);
}

#[test]
fn longer_than_72_characters() {
    let message = format!("Subject\n\n{}", "x".repeat(73));
    test_body_wider_than_72_characters(
        &message.clone(),
        &Some(Problem::new(
            ERROR.into(),
            HELP_MESSAGE.into(),
            Code::BodyWiderThan72Characters,
            &message.into(),
            Some(vec![("Too long".to_string(), 81, 1)]),
            Some("https://git-scm.com/book/en/v2/Distributed-Git-Contributing-to-a-Project#_commit_guidelines".to_string()),
        )),
    );
}

#[test]
fn first_line_ok_but_second_line_too_long() {
    let message = format!("Subject\n\nx\n{}\nx\n", "x".repeat(73));
    test_body_wider_than_72_characters(
        &message.clone(),
        &Some(Problem::new(
            ERROR.into(),
            HELP_MESSAGE.into(),
            Code::BodyWiderThan72Characters,
            &message.into(),
            Some(vec![("Too long".to_string(), 83, 1)]),
            Some("https://git-scm.com/book/en/v2/Distributed-Git-Contributing-to-a-Project#_commit_guidelines".to_string()),
        )),
    );
}

#[test]
fn last_line_included() {
    let message = format!("Subject\n\n{}", "x".repeat(73));
    test_body_wider_than_72_characters(
        &message.clone(),
        &Some(Problem::new(
            ERROR.into(),
            HELP_MESSAGE.into(),
            Code::BodyWiderThan72Characters,
            &message.into(),
            Some(vec![("Too long".to_string(), 81, 1)]),
            Some("https://git-scm.com/book/en/v2/Distributed-Git-Contributing-to-a-Project#_commit_guidelines".to_string()),
        )),
    );
}

#[test]
fn lines_after_scissors_and_comments_are_not_included() {
    let message = [
        "Subject",
        "",
        "x",
        &"x".repeat(72),
        "# ------------------------ >8 ------------------------",
        &"x".repeat(73),
    ]
    .join("\n");
    test_body_wider_than_72_characters(&message, &None);
}

fn test_body_wider_than_72_characters(message: &str, expected: &Option<Problem>) {
    let actual = &lint(&CommitMessage::from(message));
    assert_eq!(
        actual, expected,
        "Message {message:?} should have returned {expected:?}, found {actual:?}"
    );
}

#[test]
fn formatting() {
    let message = format!(
        "Subject\n\nx\n{}\nx\n{}\nx\n",
        "x".repeat(73),
        "x".repeat(80)
    );
    let problem = lint(&CommitMessage::from(message.clone()));
    let actual = fmt_report(&Report::new(problem.unwrap()));
    let expected = "BodyWiderThan72Characters (https://git-scm.com/book/en/v2/Distributed-Git-Contributing-to-a-Project#_commit_guidelines)

  x Your commit has a body wider than 72 characters
   ,-[4:73]
 3 | x
 4 | xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx
   :                                                                         |
   :                                                                         `-- Too long
 5 | x
 6 | xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx
   :                                                                         ^^^^|^^^
   :                                                                             `-- Too long
 7 | x
   `----
  help: It's important to keep the body of the commit narrower than 72
        characters because when you look at the git log, that's where it
        truncates the message. This means that people won't get the entirety
        of the information in your commit.
        
        You can fix this by making the lines in your body no more than 72
        characters
".to_string();
    assert_eq!(
        actual, expected,
        "Message {message:?} should have returned {expected:?}, found {actual:?}"
    );
}

#[test]
fn lines_after_scissors_and_comments_are_not_included_in_highlights() {
    let message = [
        "Subject",
        "",
        "x",
        &"x".repeat(73),
        "# ------------------------ >8 ------------------------",
        &"x".repeat(73),
    ]
    .join("\n");

    let problem = lint(&CommitMessage::from(message.clone()));
    let actual = fmt_report(&Report::new(problem.unwrap()));
    let expected = "BodyWiderThan72Characters (https://git-scm.com/book/en/v2/Distributed-Git-Contributing-to-a-Project#_commit_guidelines)

  x Your commit has a body wider than 72 characters
   ,-[4:73]
 3 | x
 4 | xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx
   :                                                                         |
   :                                                                         `-- Too long
 5 | # ------------------------ >8 ------------------------
   `----
  help: It's important to keep the body of the commit narrower than 72
        characters because when you look at the git log, that's where it
        truncates the message. This means that people won't get the entirety
        of the information in your commit.
        
        You can fix this by making the lines in your body no more than 72
        characters
".to_string();
    assert_eq!(
        actual, expected,
        "Message {message:?} should have returned {expected:?}, found {actual:?}"
    );
}

#[test]
fn comments_are_not_included_in_highlights() {
    let message = [
        "Subject",
        "",
        "x",
        &"x".repeat(73),
        &format!("# {}", "x".repeat(71)),
        "# ------------------------ >8 ------------------------",
        &"x".repeat(73),
    ]
    .join("\n");

    let problem = lint(&CommitMessage::from(message.clone()));
    let actual = fmt_report(&Report::new(problem.unwrap()));
    let expected = "BodyWiderThan72Characters (https://git-scm.com/book/en/v2/Distributed-Git-Contributing-to-a-Project#_commit_guidelines)

  x Your commit has a body wider than 72 characters
   ,-[4:73]
 3 | x
 4 | xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx
   :                                                                         |
   :                                                                         `-- Too long
 5 | # xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx
   `----
  help: It's important to keep the body of the commit narrower than 72
        characters because when you look at the git log, that's where it
        truncates the message. This means that people won't get the entirety
        of the information in your commit.
        
        You can fix this by making the lines in your body no more than 72
        characters
".to_string();
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

#[allow(clippy::needless_pass_by_value)]
#[quickcheck]
fn success_check(commit: String) -> TestResult {
    if commit.starts_with('\n')
        || commit
            .lines()
            .skip(1)
            .filter(|x| x.starts_with('#'))
            .any(|x| x.chars().count() > 72)
    {
        return TestResult::discard();
    }

    let message = CommitMessage::from(format!("{commit}\n# comment"));
    let result = lint(&message);
    TestResult::from_bool(result.is_none())
}

#[allow(clippy::needless_pass_by_value)]
#[quickcheck]
fn fail_check(commit: String) -> TestResult {
    if commit
        .lines()
        .filter(|x| x.starts_with('#'))
        .all(|x| x.chars().count() <= 72)
    {
        return TestResult::discard();
    }

    let message = CommitMessage::from(format!("{commit}\n# comment"));
    let result = lint(&message);
    TestResult::from_bool(result.is_none())
}
