use std::option::Option::None;

use mit_commit::CommitMessage;

use crate::model::{Code, Problem};

/// Canonical lint ID
pub(crate) const CONFIG: &str = "subject-not-separated-from-body";
/// Description of the problem
const ERROR: &str = "Your commit message is missing a blank line between the subject and the body";
/// Advice on how to correct the problem
const HELP_MESSAGE: &str = "Most tools that render and parse commit messages, expect commit \
                            messages to be in the form of subject and body. This includes git \
                            itself in tools like git-format-patch. If you don't include this you \
                            may see strange behaviour from git and any related tools.\n\nTo fix \
                            this separate subject from body with a blank line";

fn has_problem(commit_message: &CommitMessage) -> bool {
    commit_message
        .get_subject()
        .chars()
        .any(|char| '\n' == char)
}

pub(crate) fn lint(commit_message: &CommitMessage) -> Option<Problem> {
    if has_problem(commit_message) {
        let commit_text = String::from(commit_message.clone());
        let mut lines = commit_text.lines();
        let first_line_length = lines.next().map(str::len).unwrap_or_default() + 1;
        let gutter_line_length = lines.next().map(str::len).unwrap_or_default();
        Some(Problem::new(
            ERROR.into(),
            HELP_MESSAGE.into(),
            Code::SubjectNotSeparateFromBody,
            commit_message,
            Some(vec![(
                "Missing blank line".to_string(),
                first_line_length,
                gutter_line_length,
            )]),
            None,
        ))
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    #![allow(clippy::wildcard_imports)]

    use super::*;
    use crate::model::{Code, Problem};

    #[test]
    fn with_gutter() {
        test_subject_not_separate_from_body(
            "An example commit

This is an example commit
",
            &None,
        );
        test_subject_not_separate_from_body(
            "Another example

Disabling this specific lint - Co-authored

Co-authored-by: Someone Else <someone@example.com>
Co-authored-by: Someone Else <someone@example.com>
",
            &None,
        );
    }

    #[test]
    fn single_line_with_trailing_newline() {
        test_subject_not_separate_from_body("An example commit\n", &None);
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
            &None,
        );
    }

    #[test]
    fn single_line() {
        test_subject_not_separate_from_body("An example commit", &None);
    }

    #[test]
    fn gutter_missing() {
        test_subject_not_separate_from_body(
            "An example commit
This is an example commit
",
            &Some(Problem::new(
                ERROR.into(),
                HELP_MESSAGE.into(),
                Code::SubjectNotSeparateFromBody,
                &"An example commit
This is an example commit
"
                .into(),
                Some(vec![("Missing blank line".to_string(), 18_usize, 25_usize)]),
                None,
            )),
        );
        test_subject_not_separate_from_body(
            "An example commit
This is an example commit
It has more lines
It has even more lines
",
            &Some(Problem::new(
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
                None,
            )),
        );
    }

    use std::option::Option::None;

    use miette::{GraphicalReportHandler, GraphicalTheme, Report};

    #[test]
    fn formatting() {
        let message = "An example commit
This is an example commit
";
        let problem = lint(&CommitMessage::from(message.to_string()));
        let actual = fmt_report(&Report::new(problem.unwrap()));
        let expected = "SubjectNotSeparateFromBody

  \u{d7} Your commit message is missing a blank line between the subject and the
  \u{2502} body
   \u{256d}\u{2500}[1:1]
 1 \u{2502} An example commit
 2 \u{2502} This is an example commit
   \u{b7} \u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{252c}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}
   \u{b7}             \u{2570}\u{2500}\u{2500} Missing blank line
   \u{2570}\u{2500}\u{2500}\u{2500}\u{2500}
  help: Most tools that render and parse commit messages, expect commit
        messages to be in the form of subject and body. This includes git
        itself in tools like git-format-patch. If you don't include this you
        may see strange behaviour from git and any related tools.
        
        To fix this separate subject from body with a blank line
"        .to_string();
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

    fn test_subject_not_separate_from_body(message: &str, expected: &Option<Problem>) {
        let actual = &lint(&CommitMessage::from(message));
        assert_eq!(
            actual, expected,
            "Message {:?} should have returned {:?}, found {:?}",
            message, expected, actual
        );
    }
}
