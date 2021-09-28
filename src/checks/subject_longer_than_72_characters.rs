use std::option::Option::None;

use mit_commit::CommitMessage;

use crate::model::{Code, Problem};

/// Canonical lint ID
pub(crate) const CONFIG: &str = "subject-longer-than-72-characters";

/// Advice on how to correct the problem
const HELP_MESSAGE: &str = "It's important to keep the subject of the commit less than 72 \
                            characters because when you look at the git log, that's where it \
                            truncates the message. This means that people won't get the entirety \
                            of the information in your commit.\n\nPlease keep the subject line 72 \
                            characters or under";
/// Description of the problem
const ERROR: &str = "Your subject is longer than 72 characters";

const LIMIT: usize = 72;

pub(crate) fn lint(commit: &CommitMessage) -> Option<Problem> {
    if commit.get_subject().len() > LIMIT {
        Some(Problem::new(
            ERROR.into(),
            HELP_MESSAGE.into(),
            Code::SubjectLongerThan72Characters,
            commit,
            Some(vec![(
                "Too long".to_string(),
                73_usize,
                commit.get_subject().len() - LIMIT,
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

    use indoc::indoc;

    use super::*;
    use crate::model::{Code, Problem};

    #[test]
    fn shorter_than_72_characters() {
        test_subject_longer_than_72_characters(&"x".repeat(72), &None);
    }

    #[test]
    fn shorter_than_72_characters_with_a_new_line() {
        test_subject_longer_than_72_characters(&format!("{}\n", "x".repeat(72)), &None);
    }

    #[test]
    fn shorter_than_72_characters_with_realistic_trailer_and_a_body() {
        let message = indoc!(
            "
            Remove duplicated function
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


            "
        );
        test_subject_longer_than_72_characters(
            &format!("{}\n\n{}", "x".repeat(72), message),
            &None,
        );
    }

    #[test]
    fn shorter_than_72_characters_with_realistic_trailer() {
        let message = indoc!(
            "
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


            "
        );
        test_subject_longer_than_72_characters(
            &format!("{}\n\n{}", "x".repeat(72), message),
            &None,
        );
    }

    #[test]
    fn longer_than_72_characters() {
        let message = "x".repeat(73);
        test_subject_longer_than_72_characters(
            &message.clone(),
            &Some(Problem::new(
                ERROR.into(),
                HELP_MESSAGE.into(),
                Code::SubjectLongerThan72Characters,
                &message.into(),
                Some(vec![("Too long".to_string(), 73_usize, 1_usize)]),
                None,
            )),
        );
    }

    #[test]
    fn longer_than_72_characters_and_a_newline() {
        let message = format!("{}\n", "x".repeat(73));
        test_subject_longer_than_72_characters(
            &message.clone(),
            &Some(Problem::new(
                ERROR.into(),
                HELP_MESSAGE.into(),
                Code::SubjectLongerThan72Characters,
                &message.into(),
                Some(vec![("Too long".to_string(), 73_usize, 1_usize)]),
                None,
            )),
        );
    }

    #[test]
    fn longer_than_72_characters_and_a_body() {
        let message = indoc!(
            "
            Remove duplicated function
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


            "
        );
        let message = format!("{}\n\n{}", "x".repeat(73), message);
        test_subject_longer_than_72_characters(
            &message.clone(),
            &Some(Problem::new(
                ERROR.into(),
                HELP_MESSAGE.into(),
                Code::SubjectLongerThan72Characters,
                &message.into(),
                Some(vec![("Too long".to_string(), 73_usize, 1_usize)]),
                None,
            )),
        );
    }

    #[test]
    fn longer_than_72_characters_with_realistic_tail() {
        let message = indoc!(
            "
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


            "
        );
        test_subject_longer_than_72_characters(
            &format!("{}\n\n{}", "x".repeat(72), message),
            &None,
        );
    }

    #[test]
    fn formatting() {
        let message = "x".repeat(73);
        let problem = lint(&CommitMessage::from(message.to_string()));
        let actual = fmt_report(&Report::new(problem.unwrap()));
        let expected = indoc!(
            "
            SubjectLongerThan72Characters
            
              \u{d7} Your subject is longer than 72 characters
               \u{256d}\u{2500}\u{2500}\u{2500}\u{2500}
             1 \u{2502} xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx
               \u{2570}\u{2500}\u{2500}\u{2500}\u{2500}
              help: It's important to keep the subject of the commit less than 72
                    characters because when you look at the git log, that's where it
                    truncates the message. This means that people won't get the entirety
                    of the information in your commit.
                    
                    Please keep the subject line 72 characters or under
            "
        )
        .to_string();
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
    use std::option::Option::None;

    use miette::{GraphicalReportHandler, GraphicalTheme, Report};

    fn test_subject_longer_than_72_characters(message: &str, expected: &Option<Problem>) {
        let actual = &lint(&CommitMessage::from(message));
        assert_eq!(
            actual, expected,
            "Message {:?} should have returned {:?}, found {:?}",
            message, expected, actual
        );
    }
}
