use std::{collections::BTreeMap, option::Option::None};

use crate::model::{Code, Problem, ProblemBuilder};
use mit_commit::{CommitMessage, Trailer};

/// Canonical lint ID
pub const CONFIG: &str = "duplicated-trailers";

/// Description of the problem
pub const ERROR: &str = "Your commit message has duplicated trailers";

const TRAILERS_TO_CHECK_FOR_DUPLICATES: [&str; 3] =
    ["Signed-off-by", "Co-authored-by", "Relates-to"];
const FIELD_SINGULAR: &str = "field";
const FIELD_PLURAL: &str = "fields";

/// Configuration for duplicated trailers linting
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct DuplicatedTrailersConfig {
    /// Trailers to check for duplicates
    pub trailers_to_check: Vec<String>,
}

impl Default for DuplicatedTrailersConfig {
    fn default() -> Self {
        Self {
            trailers_to_check: TRAILERS_TO_CHECK_FOR_DUPLICATES
                .iter()
                .map(|&s| s.to_string())
                .collect(),
        }
    }
}

/// Get a list of duplicated trailers from a commit message
///
/// # Arguments
///
/// * `commit_message` - The commit message to check for duplicated trailers
/// * `trailers_to_check` - List of trailer keys to check for duplicates
///
/// # Returns
///
/// A vector of strings containing the keys of duplicated trailers
fn get_duplicated_trailers(
    commit_message: &CommitMessage<'_>,
    trailers_to_check: &[String],
) -> Vec<String> {
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

            if count > 1 && trailers_to_check.contains(&key) {
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
pub fn lint(commit: &CommitMessage<'_>) -> Option<Problem> {
    lint_with_config(commit, &DuplicatedTrailersConfig::default())
}

fn lint_with_config(
    commit: &CommitMessage<'_>,
    config: &DuplicatedTrailersConfig,
) -> Option<Problem> {
    Some(commit)
        .filter(|commit| has_problem(commit, &config.trailers_to_check))
        .map(|commit| create_problem(commit, &config.trailers_to_check))
}

fn has_problem(commit: &CommitMessage<'_>, trailers_to_check: &[String]) -> bool {
    !get_duplicated_trailers(commit, trailers_to_check).is_empty()
}

fn create_problem(commit: &CommitMessage, trailers_to_check: &[String]) -> Problem {
    let duplicated_trailers = get_duplicated_trailers(commit, trailers_to_check);

    // We need to clone here as String::from works with CommitMessage but not &CommitMessage
    let commit_message = String::from(commit.clone());
    let warning = warning(&duplicated_trailers);

    // Create labels for all duplicated trailers using iterator-based line scanning.
    // We need to track byte offsets manually since str::split doesn't provide them.
    // The iterator feeds directly into the fold, avoiding an intermediate collection.
    duplicated_trailers
        .iter()
        .flat_map(|trailer| {
            let prefix = format!("{trailer}: ");
            let mut occurrence_count = 0;
            commit_message
                .split('\n')
                .scan(0usize, move |offset, line| {
                    let start = *offset;
                    let label = if line.starts_with(&prefix) {
                        occurrence_count += 1;
                        if occurrence_count > 1 {
                            Some((format!("Duplicated `{trailer}`"), start, line.len()))
                        } else {
                            None
                        }
                    } else {
                        None
                    };
                    *offset += line.len() + 1; // +1 for the '\n'
                    Some(label)
                })
                .flatten()
        })
        .fold(
            ProblemBuilder::new(ERROR, warning, Code::DuplicatedTrailers, commit)
                .with_url("https://git-scm.com/docs/githooks#_commit_msg"),
            |builder, (label, position, length)| builder.with_label(label, position, length),
        )
        .build()
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

    use miette::{Diagnostic, GraphicalReportHandler, GraphicalTheme, Report};
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

    #[test]
    fn test_duplicate_checked_trailer_is_flagged() {
        // Direct (non-quickcheck) confirmation that a duplicated checked
        // trailer key is flagged — the behaviour the quickcheck property
        // should actually exercise.
        let message = "Subject\n\nBody text\n\nSigned-off-by: Billie <billie@example.com>\nSigned-off-by: Billie <billie@example.com>\n";
        let result = lint(&CommitMessage::from(message));
        assert!(
            result.is_some(),
            "Duplicated Signed-off-by trailer should be flagged"
        );
    }

    #[allow(clippy::needless_pass_by_value)]
    #[quickcheck]
    fn test_quickcheck_duplicate_trailers_fail(
        commit: String,
        trailer_tag: String,
        trailer_text: String,
        repeats: usize,
    ) -> TestResult {
        // The lint only flags duplicates of these three trailer keys. The old
        // guard required the tag to be ASCII-alphanumeric, which excluded all
        // three checked keys (they contain hyphens). Combined with an equally
        // aggressive guard on trailer_text, virtually every random input was
        // discarded and the property never actually exercised duplicate
        // detection — giving false confidence.
        const CHECKED_KEYS: [&str; 3] = ["Signed-off-by", "Co-authored-by", "Relates-to"];
        let trailer_tag = CHECKED_KEYS[trailer_tag.len() % CHECKED_KEYS.len()];

        // Sanitise the trailer value: strip characters that would break the
        // trailer line, but allow most Unicode (unlike the old ASCII-only guard).
        let trailer_text: String = trailer_text
            .chars()
            .filter(|c| !matches!(c, '\n' | '\r' | '\0'))
            .collect();
        if trailer_text.is_empty() || trailer_text.len() > 10000 {
            return TestResult::discard();
        }

        if repeats > 50 {
            return TestResult::discard();
        }

        // Discard if the commit body contains a scissors marker, as trailers
        // after it would be ignored by the parser.
        if commit.contains(">8") || commit.contains("8<") {
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

    #[test]
    fn test_duplicate_trailers_label_position_with_multibyte_chars() {
        // Multi-byte UTF-8 chars (ä, ö, ü) in the body before trailers
        let message = "Beispiel-Commit\n\nÜberprüfung der Änderungen im Börsenbereich.\n\nSigned-off-by: Billie Thompson <email@example.com>\nSigned-off-by: Billie Thompson <email@example.com>\n";
        let problem = lint(&CommitMessage::from(message.to_string())).unwrap();

        // Get the labels from the diagnostic
        let labels: Vec<_> = problem
            .labels()
            .unwrap()
            .map(|span| (span.label().unwrap().to_string(), span.offset(), span.len()))
            .collect();

        // There should be exactly 1 label for the duplicated Signed-off-by
        assert_eq!(labels.len(), 1, "Expected 1 label, got {labels:?}");
        let (label_text, offset, len) = &labels[0];
        assert_eq!(label_text, "Duplicated `Signed-off-by`");

        // The label offset should point to the byte position of the second "Signed-off-by"
        let expected_byte_offset = message.find("Signed-off-by").unwrap();
        let second_occurrence = message[expected_byte_offset + 1..]
            .find("Signed-off-by")
            .unwrap()
            + expected_byte_offset
            + 1;
        assert_eq!(
            *offset, second_occurrence,
            "Label offset should be byte position of second trailer"
        );

        // The label length should cover the full trailer line in bytes
        let trailer_line = "Signed-off-by: Billie Thompson <email@example.com>";
        assert_eq!(
            *len,
            trailer_line.len(),
            "Label length should be byte length of trailer line, got {} (expected {})",
            len,
            trailer_line.len()
        );
    }

    #[test]
    fn test_match_indices_false_positive_in_body() {
        // The body contains "Signed-off-by" as plain text, not as a trailer
        // There is one actual duplicate trailer
        let message = "An example commit\n\nPlease Signed-off-by here\n\nSigned-off-by: Billie Thompson <email@example.com>\nSigned-off-by: Billie Thompson <email@example.com>\n";
        let problem = lint(&CommitMessage::from(message.to_string()));

        // Should still detect the problem
        assert!(problem.is_some());

        // But should only have ONE label (for the actual duplicate)
        let labels: Vec<_> = problem
            .unwrap()
            .labels()
            .unwrap()
            .map(|span| (span.label().unwrap().to_string(), span.offset(), span.len()))
            .collect();

        assert_eq!(
            labels.len(),
            1,
            "Expected 1 label for the single duplicate trailer, but got {labels:?}"
        );
    }

    #[test]
    fn test_match_indices_false_positive_trailer_value_contains_key() {
        // A trailer value contains another trailer key
        let message = "An example commit\n\nSigned-off-by: Co-authored-by <email@example.com>\nCo-authored-by: Billie Thompson <email@example.com>\nCo-authored-by: Billie Thompson <email@example.com>\n";
        let problem = lint(&CommitMessage::from(message.to_string()));

        // Should detect the problem
        assert!(problem.is_some());

        // Should only have ONE label for the duplicated Co-authored-by trailer
        let labels: Vec<_> = problem
            .unwrap()
            .labels()
            .unwrap()
            .map(|span| (span.label().unwrap().to_string(), span.offset(), span.len()))
            .collect();

        assert_eq!(
            labels.len(),
            1,
            "Expected 1 label for the single duplicate trailer, but got {labels:?}"
        );
    }
}
