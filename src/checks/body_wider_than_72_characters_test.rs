#![allow(clippy::wildcard_imports)]

use std::option::Option::None;

use miette::{GraphicalReportHandler, GraphicalTheme, Report};
use mit_commit::CommitMessage;
use quickcheck::TestResult;

use super::body_wider_than_72_characters::{lint, ERROR, HELP_MESSAGE};
use crate::{model::Code, Problem};

#[test]
fn narrower_than_72_characters() {
    test_body_wider_than_72_characters(&format!("Subject\n\n{}", "x".repeat(72)), None);
}

#[test]
fn no_body() {
    test_body_wider_than_72_characters("Subject", None);
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
    test_body_wider_than_72_characters(&format!("{}\n\n{message}", "x".repeat(72)), None);
}

#[test]
fn longer_than_72_characters() {
    let message = format!("Subject\n\n{}", "x".repeat(73));
    test_body_wider_than_72_characters(
        &message.clone(),
        Some(Problem::new(
            ERROR.into(),
            HELP_MESSAGE.into(),
            Code::BodyWiderThan72Characters,
            &message.clone().into(),
            Some(vec![("Too long".to_string(), 81, 1)]), // Correct offset after 8 bytes for "Subject\n\n"
            Some("https://git-scm.com/book/en/v2/Distributed-Git-Contributing-to-a-Project#_commit_guidelines".to_string()),
        )).as_ref(),
    );
}

#[test]
fn longer_than_73_still_fails() {
    let message = format!("Subject\n\n{}", "x".repeat(75));
    test_body_wider_than_72_characters(
        &message.clone(),
        Some(Problem::new(
            ERROR.into(),
            HELP_MESSAGE.into(),
            Code::BodyWiderThan72Characters,
            &message.clone().into(),
            Some(vec![("Too long".to_string(), 81, 3)]),
            Some("https://git-scm.com/book/en/v2/Distributed-Git-Contributing-to-a-Project#_commit_guidelines".to_string()),
        )).as_ref(),
    );
}

#[test]
fn multiple_long_lines_fails() {
    let message = format!("Subject\n\n{}\n{}", "x".repeat(73), "y".repeat(73));
    test_body_wider_than_72_characters(
        &message.clone(),
        Some(Problem::new(
            ERROR.into(),
            HELP_MESSAGE.into(),
            Code::BodyWiderThan72Characters,
            &message.clone().into(),
            Some(vec![("Too long".to_string(), 81, 1), ("Too long".to_string(), 155, 1)]),
            Some("https://git-scm.com/book/en/v2/Distributed-Git-Contributing-to-a-Project#_commit_guidelines".to_string()),
        )).as_ref(),
    );
}

#[test]
fn first_line_ok_but_second_line_too_long() {
    let message = format!("Subject\n\nx\n{}\nx\n", "x".repeat(73));
    test_body_wider_than_72_characters(
        &message.clone(),
        Some(Problem::new(
            ERROR.into(),
            HELP_MESSAGE.into(),
            Code::BodyWiderThan72Characters,
            &message.clone().into(),
            Some(vec![("Too long".to_string(), 83, 1)]),
            Some("https://git-scm.com/book/en/v2/Distributed-Git-Contributing-to-a-Project#_commit_guidelines".to_string()),
        )).as_ref(),
    );
}

#[test]
fn last_line_included() {
    let message = format!("Subject\n\n{}", "x".repeat(73));
    test_body_wider_than_72_characters(
        &message.clone(),
        Some(Problem::new(
            ERROR.into(),
            HELP_MESSAGE.into(),
            Code::BodyWiderThan72Characters,
            &message.into(),
            Some(vec![("Too long".to_string(), 81, 1)]),
            Some("https://git-scm.com/book/en/v2/Distributed-Git-Contributing-to-a-Project#_commit_guidelines".to_string()),
        )).as_ref(),
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
    test_body_wider_than_72_characters(&message, None);
}

fn test_body_wider_than_72_characters(message: &str, expected: Option<&Problem>) {
    let actual = lint(&CommitMessage::from(message));
    assert_eq!(
        actual.as_ref(),
        expected,
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
fn success_check(input: Vec<u8>) -> TestResult {
    // Clean and normalize test input through several transformations:
    // 1. Convert raw bytes to valid UTF-8, replacing invalid sequences
    let utf8_cleaned = String::from_utf8_lossy(&input).into_owned();

    // Ensure we have a valid commit structure with non-empty subject and body separator
    if utf8_cleaned.is_empty() || utf8_cleaned.starts_with('\n') || !utf8_cleaned.contains("\n\n") {
        return TestResult::discard();
    }

    // Split into subject and body parts
    let parts: Vec<&str> = utf8_cleaned.split("\n\n").collect();
    if parts.len() != 2 || parts[0].trim().is_empty() {
        return TestResult::discard();
    }

    // Check body lines (excluding comments) for length
    let body = parts[1];
    let mut lines_valid = true;

    for line in body.split('\n') {
        // Skip comment lines
        if line.starts_with('#') {
            continue;
        }

        // Check actual byte length like the linter does
        if line.len() > 72 {
            lines_valid = false;
            break;
        }
    }

    if !lines_valid {
        return TestResult::discard();
    }

    let message = CommitMessage::from(utf8_cleaned);
    let result = lint(&message);
    TestResult::from_bool(result.is_none())
}

#[test]
fn handles_unicode_characters_correctly() {
    // This string has 73 Unicode characters in a single line (146 bytes)
    let message = format!("Subject\n\n{}", "\u{1f600}".repeat(73));
    test_body_wider_than_72_characters(
        &message,
        Some(Problem::new(
            ERROR.into(),
            HELP_MESSAGE.into(),
            Code::BodyWiderThan72Characters,
            &message.clone().into(),
            Some(vec![("Too long".to_string(), 301, 1)]),
            Some("https://git-scm.com/book/en/v2/Distributed-Git-Contributing-to-a-Project#_commit_guidelines".to_string()),
        )).as_ref(),
    );
}

#[test]
fn handles_null_bytes_correctly() {
    let message = "\0\n\n\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0";
    let expected_problem = Problem::new(
        ERROR.into(),
        HELP_MESSAGE.into(),
        Code::BodyWiderThan72Characters,
        &CommitMessage::from(message),
        Some(vec![("Too long".to_string(), 75, 1)]),
        Some("https://git-scm.com/book/en/v2/Distributed-Git-Contributing-to-a-Project#_commit_guidelines".to_string()),
    );
    test_body_wider_than_72_characters(message, Some(&expected_problem));
}

#[derive(Debug, Clone)]
struct CommitBody(String);

impl quickcheck::Arbitrary for CommitBody {
    fn arbitrary(g: &mut quickcheck::Gen) -> Self {
        // Generate body lines with some lines over 72 chars but no invalid characters
        let line_count = usize::arbitrary(g) % 20 + 1;
        let mut body = String::new();
        
        for _ in 0..line_count {
            // Generate valid commit body lines with printable ASCII chars only
            let mut line = String::arbitrary(g)
                .replace(|c: char| !c.is_ascii_graphic() && c != ' ', "") // Remove non-printables
                .replace("\n", " ")
                .trim() // Remove leading/trailing whitespace
                .to_string();

            // Ensure at least one line exceeds limit if we're supposed to fail
            if bool::arbitrary(g) || line.is_empty() {
                // Add overlong text at start/middle/end randomly
                let position = g.choose(&["start", "middle", "end"]).unwrap();
                let padding = " ".repeat(*g.choose((0..=72).collect::<Vec<_>>().as_slice()).unwrap());
                
                match position {
                    "start" => line = format!("{}{}", "x".repeat(73), padding),
                    "end" => line = format!("{}{}", padding, "x".repeat(73)),
                    _ => line = format!("{}{}{}", 
                        padding, "x".repeat(73), padding),
                }
            }
            
            body.push_str(&line);
            body.push('\n');
        }
        
        // Build full commit message with valid structure
        CommitBody(format!("Valid subject\n\n{}", body.trim_end()))
    }
}

#[allow(clippy::needless_pass_by_value)]
#[quickcheck]
fn fail_check(commit: CommitBody) -> TestResult {
    let commit = commit.0;

    // Split into subject and body parts
    let parts: Vec<&str> = commit.split("\n\n").collect();
    if parts.len() != 2 || parts[0].trim().is_empty() {
        return TestResult::discard();
    }

    // Check body lines (excluding comments) for at least one line over CHARACTER limit
    let body = parts[1];
    if body
        .lines()
        .filter(|line| !line.starts_with('#'))
        .all(|line| line.chars().count() <= 72)
    {
        return TestResult::discard();
    }

    let message = CommitMessage::from(commit);
    let result = lint(&message);
    TestResult::from_bool(result.is_some())
}
