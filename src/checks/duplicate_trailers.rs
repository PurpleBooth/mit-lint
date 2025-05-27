use std::{collections::BTreeMap, option::Option::None};

use crate::model::{Code, Problem};
use mit_commit::{CommitMessage, Trailer};

/// Canonical lint ID
pub const CONFIG: &str = "duplicated-trailers";

/// Description of the problem
pub const ERROR: &str = "Your commit message has duplicated trailers";

const TRAILERS_TO_CHECK_FOR_DUPLICATES: [&str; 3] =
    ["Signed-off-by", "Co-authored-by", "Relates-to"];
const FIELD_SINGULAR: &str = "field";
const FIELD_PLURAL: &str = "fields";

/// Get a list of duplicated trailers from a commit message
///
/// # Arguments
///
/// * `commit_message` - The commit message to check for duplicated trailers
///
/// # Returns
///
/// A vector of strings containing the keys of duplicated trailers
fn get_duplicated_trailers(commit_message: &CommitMessage<'_>) -> Vec<String> {
    commit_message
        .get_trailers()
        .iter()
        .fold(
            BTreeMap::new(),
            |mut acc: BTreeMap<&Trailer<'_>, usize>, trailer| {
                let count = acc.get(trailer).map_or(1, |c| c + 1);
                acc.insert(trailer, count);
                acc
            },
        )
        .into_iter()
        .filter_map(|(trailer, count)| {
            let key = trailer.get_key();

            if count > 1 && TRAILERS_TO_CHECK_FOR_DUPLICATES.contains(&key.as_str()) {
                Some(key)
            } else {
                None
            }
        })
        .collect::<Vec<_>>()
}

/// Checks if the commit message contains duplicated trailers
///
/// # Arguments
///
/// * `commit` - The commit message to check
///
/// # Returns
///
/// * `Some(Problem)` - If the commit message contains duplicated trailers
/// * `None` - If the commit message does not contain duplicated trailers
///
/// # Examples
///
/// ```rust
/// use mit_commit::CommitMessage;
/// use mit_lint::Lint::DuplicatedTrailers;
///
/// // This should pass
/// let passing = CommitMessage::from("Subject\n\nBody\n\nSigned-off-by: Someone <someone@example.com>");
/// assert!(DuplicatedTrailers.lint(&passing).is_none());
///
/// // This should fail
/// let failing = CommitMessage::from(
///     "Subject\n\nBody\n\nSigned-off-by: Someone <someone@example.com>\nSigned-off-by: Someone <someone@example.com>"
/// );
/// assert!(DuplicatedTrailers.lint(&failing).is_some());
/// ```
///
/// # Errors
///
/// This function will never return an error, only an Option<Problem>
pub fn lint(commit: &CommitMessage<'_>) -> Option<Problem> {
    let duplicated_trailers = get_duplicated_trailers(commit);

    if duplicated_trailers.is_empty() {
        return None;
    }

    // We need to clone here as String::from works with CommitMessage but not &CommitMessage
    let commit_message = String::from(commit.clone());
    let warning = warning(&duplicated_trailers);

    // Create labels for all duplicated trailers
    let labels = duplicated_trailers
        .into_iter()
        .flat_map(|trailer| {
            // First, collect all positions where the trailer appears
            let positions: Vec<_> = commit_message
                .match_indices(&trailer)
                .skip(1) // Skip the first occurrence as it's not a duplicate
                .collect();

            // Then, calculate line lengths for each position
            let mut results = Vec::new();
            for (position, _) in positions {
                let line_length = commit_message
                    .chars()
                    .skip(position)
                    .take_while(|x| x != &'\n')
                    .count();

                results.push((format!("Duplicated `{trailer}`"), position, line_length));
            }

            results
        })
        .collect();

    Some(Problem::new(
        ERROR.into(),
        warning,
        Code::DuplicatedTrailers,
        commit,
        Some(labels),
        Some("https://git-scm.com/docs/githooks#_commit_msg".to_string()),
    ))
}

/// Generate a warning message for duplicated trailers
///
/// # Arguments
///
/// * `duplicated_trailers` - A slice of strings containing the keys of duplicated trailers
///
/// # Returns
///
/// A string containing a warning message about the duplicated trailers
fn warning(duplicated_trailers: &[String]) -> String {
    format!(
        "These are normally added accidentally when you're rebasing or amending to a commit, \
         sometimes in the text editor, but often by git hooks.\n\nYou can fix this by deleting \
         the duplicated \"{}\" {}",
        duplicated_trailers.join("\", \""),
        if duplicated_trailers.len() > 1 {
            FIELD_PLURAL
        } else {
            FIELD_SINGULAR
        }
    )
}

#[cfg(test)]
mod tests {
    use std::option::Option::None;

    use miette::{GraphicalReportHandler, GraphicalTheme, Report};
    use mit_commit::CommitMessage;
    use quickcheck::TestResult;

    use super::*;
    use crate::{Problem, model::Code};

    #[test]
    fn test_commit_without_trailers_passes() {
        test_lint_duplicated_trailers(
            "An example commit

This is an example commit without any duplicate trailers
"
            .into(),
            None,
        );
    }

    #[test]
    fn test_commit_with_multiple_duplicate_trailers_fails() {
        let message = "An example commit

This is an example commit without any duplicate trailers

Signed-off-by: Billie Thompson <email@example.com>
Signed-off-by: Billie Thompson <email@example.com>
Co-authored-by: Billie Thompson <email@example.com>
Co-authored-by: Billie Thompson <email@example.com>
";
        test_lint_duplicated_trailers(
            message.into(),
            Some(Problem::new(
                ERROR.into(),
                "These are normally added accidentally when you\'re rebasing or amending to a \
                 commit, sometimes in the text editor, but often by git hooks.\n\nYou can fix \
                 this by deleting the duplicated \"Co-authored-by\", \"Signed-off-by\" fields"
                    .into(),
                Code::DuplicatedTrailers,
                &message.into(),
                Some(vec![
                    (
                        "Duplicated `Co-authored-by`".to_string(),
                        231_usize,
                        51_usize,
                    ),
                    (
                        "Duplicated `Signed-off-by`".to_string(),
                        128_usize,
                        50_usize,
                    ),
                ]),
                Some(
                    "https://git-scm.com/docs/githooks#_commit_msg"
                        .parse()
                        .unwrap(),
                ),
            ))
            .as_ref(),
        );
    }

    #[test]
    fn test_duplicate_signed_off_by_trailers_fails() {
        let message = "An example commit

This is an example commit without any duplicate trailers

Signed-off-by: Billie Thompson <email@example.com>
Signed-off-by: Billie Thompson <email@example.com>
";
        test_lint_duplicated_trailers(
            message.into(),
            Some(Problem::new(
                ERROR.into(),
                "These are normally added accidentally when you\'re rebasing or amending to a \
                 commit, sometimes in the text editor, but often by git hooks.\n\nYou can fix \
                 this by deleting the duplicated \"Signed-off-by\" field"
                    .into(),
                Code::DuplicatedTrailers,
                &message.into(),
                Some(vec![("Duplicated `Signed-off-by`".to_string(), 128, 50)]),
                Some(
                    "https://git-scm.com/docs/githooks#_commit_msg"
                        .parse()
                        .unwrap(),
                ),
            ))
            .as_ref(),
        );
    }

    #[test]
    fn test_duplicate_co_authored_by_trailers_fails() {
        let message = "An example commit

This is an example commit without any duplicate trailers

Co-authored-by: Billie Thompson <email@example.com>
Co-authored-by: Billie Thompson <email@example.com>
";
        test_lint_duplicated_trailers(
            message.into(),
            Some(Problem::new(
                ERROR.into(),
                "These are normally added accidentally when you\'re rebasing or amending to a \
                 commit, sometimes in the text editor, but often by git hooks.\n\nYou can fix \
                 this by deleting the duplicated \"Co-authored-by\" field"
                    .into(),
                Code::DuplicatedTrailers,
                &message.into(),
                Some(vec![("Duplicated `Co-authored-by`".to_string(), 129, 51)]),
                Some("https://git-scm.com/docs/githooks#_commit_msg".to_string()),
            ))
            .as_ref(),
        );
    }

    #[test]
    fn test_duplicate_relates_to_trailers_fails() {
        let message = "An example commit

This is an example commit without any duplicate trailers

Relates-to: #315
Relates-to: #315
";
        test_lint_duplicated_trailers(
            message.into(),
            Some(Problem::new(
                ERROR.into(),
                "These are normally added accidentally when you\'re rebasing or amending to a \
                 commit, sometimes in the text editor, but often by git hooks.\n\nYou can fix \
                 this by deleting the duplicated \"Relates-to\" field"
                    .into(),
                Code::DuplicatedTrailers,
                &message.into(),
                Some(vec![("Duplicated `Relates-to`".to_string(), 94, 16)]),
                Some("https://git-scm.com/docs/githooks#_commit_msg".to_string()),
            ))
            .as_ref(),
        );
    }

    #[test]
    fn test_duplicate_trailers_in_scissors_section_are_ignored() {
        test_lint_duplicated_trailers(
            "Move to specdown
# Bitte geben Sie eine Commit-Beschreibung fur Ihre Anderungen ein. Zeilen,
# die mit '#' beginnen, werden ignoriert, und eine leere Beschreibung

# ------------------------ >8 ------------------------
# Andern oder entfernen Sie nicht die obige Zeile.
# Alles unterhalb von ihr wird ignoriert.
diff --git a/Makefile b/Makefile
index 0d3fc98..38a2784 100644
--- a/Makefile
+++ b/Makefile
+
 This is a commit message that has trailers and is invalid

-Signed-off-by: Someone Else <someone@example.com>
-Signed-off-by: Someone Else <someone@example.com>
 Co-authored-by: Billie Thompson <billie@example.com>
 Co-authored-by: Billie Thompson <billie@example.com>
+Signed-off-by: Someone Else <someone@example.com>
+Signed-off-by: Someone Else <someone@example.com>


 ---
@@ -82,6 +82,7 @@ Co-authored-by: Billie Thompson <billie@example.com>
 Your commit message has duplicated trailers

 You can fix this by deleting the duplicated \"Signed-off-by\", \"Co-authored-by\" \
 fields
+
"
            .into(),
            None,
        );
    }

    #[test]
    fn test_duplicate_non_standard_trailers_are_allowed() {
        test_lint_duplicated_trailers(
            "An example commit

This is an example commit without any duplicate trailers

Anything: Billie Thompson <email@example.com>
Anything: Billie Thompson <email@example.com>
"
            .into(),
            None,
        );
    }

    fn test_lint_duplicated_trailers(message: String, expected: Option<&Problem>) {
        let actual = lint(&CommitMessage::from(message));
        assert_eq!(
            actual.as_ref(),
            expected,
            "Expected {expected:?}, found {actual:?}"
        );
    }

    #[test]
    fn test_error_report_formatting() {
        let message = "An example commit

This is an example commit without any duplicate trailers

Signed-off-by: Billie Thompson <email@example.com>
Signed-off-by: Billie Thompson <email@example.com>
Co-authored-by: Billie Thompson <email@example.com>
Co-authored-by: Billie Thompson <email@example.com>
";
        let problem = lint(&CommitMessage::from(message.to_string()));
        let actual = fmt_report(&Report::new(problem.unwrap()));
        let expected = "DuplicatedTrailers (https://git-scm.com/docs/githooks#_commit_msg)

  x Your commit message has duplicated trailers
   ,-[6:1]
 5 | Signed-off-by: Billie Thompson <email@example.com>
 6 | Signed-off-by: Billie Thompson <email@example.com>
   : ^^^^^^^^^^^^^^^^^^^^^^^^^|^^^^^^^^^^^^^^^^^^^^^^^^
   :                          `-- Duplicated `Signed-off-by`
 7 | Co-authored-by: Billie Thompson <email@example.com>
 8 | Co-authored-by: Billie Thompson <email@example.com>
   : ^^^^^^^^^^^^^^^^^^^^^^^^^|^^^^^^^^^^^^^^^^^^^^^^^^^
   :                          `-- Duplicated `Co-authored-by`
   `----
  help: These are normally added accidentally when you're rebasing or amending
        to a commit, sometimes in the text editor, but often by git hooks.
        
        You can fix this by deleting the duplicated \"Co-authored-by\", \"Signed-
        off-by\" fields
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

    #[allow(clippy::needless_pass_by_value)]
    #[quickcheck]
    fn test_quickcheck_duplicate_trailers_fail(
        commit: String,
        trailer_tag: String,
        trailer_text: String,
        repeats: usize,
    ) -> TestResult {
        if trailer_tag.len() > 10000
            || trailer_tag.is_empty()
            || trailer_tag.chars().any(|x| !x.is_ascii_alphanumeric())
        {
            return TestResult::discard();
        }
        if trailer_text.len() > 10000
            || trailer_text.is_empty()
            || trailer_text.chars().any(|x| !x.is_ascii_alphanumeric())
        {
            return TestResult::discard();
        }

        if repeats > 50 {
            return TestResult::discard();
        }

        let message = CommitMessage::from(format!(
            "{}\n\n{}",
            commit,
            format!("{trailer_tag}: {trailer_text}\n").repeat(repeats.saturating_add(2))
        ));
        let result = lint(&message);
        TestResult::from_bool(result.is_some())
    }

    #[allow(clippy::needless_pass_by_value)]
    #[quickcheck]
    fn test_quickcheck_no_duplicate_trailers_pass(commit: String) -> TestResult {
        let message = CommitMessage::from(commit);
        let result = lint(&message);
        TestResult::from_bool(result.is_none())
    }
}
