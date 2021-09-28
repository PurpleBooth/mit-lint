use std::{collections::BTreeMap, ops::Add, option::Option::None};

use mit_commit::{CommitMessage, Trailer};

use crate::model::{Code, Problem};

/// Canonical lint ID
pub(crate) const CONFIG: &str = "duplicated-trailers";

const TRAILERS_TO_CHECK_FOR_DUPLICATES: [&str; 3] =
    ["Signed-off-by", "Co-authored-by", "Relates-to"];
const FIELD_SINGULAR: &str = "field";
/// Description of the problem
const ERROR: &str = "Your commit message has duplicated trailers";

const FIELD_PLURAL: &str = "fields";

fn get_duplicated_trailers(commit_message: &CommitMessage) -> Vec<String> {
    commit_message
        .get_trailers()
        .iter()
        .fold(
            BTreeMap::new(),
            |acc: BTreeMap<&Trailer, usize>, trailer| {
                let mut next = acc.clone();
                match acc.get(trailer) {
                    Some(count) => next.insert(trailer, count.add(1)),
                    None => next.insert(trailer, 1),
                };

                next
            },
        )
        .into_iter()
        .filter_map(|(trailer, usize)| {
            let key: &str = &trailer.get_key();

            if usize > 1 && TRAILERS_TO_CHECK_FOR_DUPLICATES.contains(&key) {
                Some(trailer.get_key())
            } else {
                None
            }
        })
        .collect::<Vec<_>>()
}

pub(crate) fn lint(commit: &CommitMessage) -> Option<Problem> {
    let duplicated_trailers = get_duplicated_trailers(commit);
    if duplicated_trailers.is_empty() {
        None
    } else {
        let commit_text = String::from(commit.clone());
        let warning = warning(&duplicated_trailers);
        Some(Problem::new(
            ERROR.into(),
            warning,
            Code::DuplicatedTrailers,
            commit,
            Some(
                duplicated_trailers
                    .into_iter()
                    .flat_map(|trailer| {
                        commit_text
                            .match_indices(&trailer)
                            .skip(1)
                            .map(|x| {
                                (
                                    format!("Duplicated `{}`", trailer),
                                    x.0,
                                    commit_text
                                        .chars()
                                        .skip(x.0)
                                        .take_while(|x| x != &'\n')
                                        .count(),
                                )
                            })
                            .collect::<Vec<_>>()
                    })
                    .collect(),
            ),
            None,
        ))
    }
}

fn warning(duplicated_trailers: &[String]) -> String {
    let warning = format!(
        "These are normally added accidentally when you're rebasing or amending to a commit, \
         sometimes in the text editor, but often by git hooks.\n\nYou can fix this by deleting \
         the duplicated \"{}\" {}",
        duplicated_trailers.join("\", \""),
        if duplicated_trailers.len() > 1 {
            FIELD_PLURAL
        } else {
            FIELD_SINGULAR
        }
    );
    warning
}

#[cfg(test)]
mod tests_has_duplicated_trailers {
    #![allow(clippy::wildcard_imports)]

    use std::option::Option::None;

    use miette::{GraphicalReportHandler, GraphicalTheme, Report};
    use mit_commit::CommitMessage;

    use super::*;
    use crate::model::Code;

    #[test]
    fn commit_without_trailers() {
        test_lint_duplicated_trailers(
            "An example commit

This is an example commit without any duplicate trailers
"
            .into(),
            &None,
        );
    }

    #[test]
    fn two_duplicates() {
        let message = "An example commit

This is an example commit without any duplicate trailers

Signed-off-by: Billie Thompson <email@example.com>
Signed-off-by: Billie Thompson <email@example.com>
Co-authored-by: Billie Thompson <email@example.com>
Co-authored-by: Billie Thompson <email@example.com>
";
        test_lint_duplicated_trailers(
            message.into(),
            &Some(Problem::new(
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
                None,
            )),
        );
    }

    #[test]
    fn signed_off_by() {
        let message = "An example commit

This is an example commit without any duplicate trailers

Signed-off-by: Billie Thompson <email@example.com>
Signed-off-by: Billie Thompson <email@example.com>
";
        test_lint_duplicated_trailers(
            message.into(),
            &Some(Problem::new(
                ERROR.into(),
                "These are normally added accidentally when you\'re rebasing or amending to a \
                 commit, sometimes in the text editor, but often by git hooks.\n\nYou can fix \
                 this by deleting the duplicated \"Signed-off-by\" field"
                    .into(),
                Code::DuplicatedTrailers,
                &message.into(),
                Some(vec![("Duplicated `Signed-off-by`".to_string(), 128, 50)]),
                None,
            )),
        );
    }

    #[test]
    fn co_authored_by() {
        let message = "An example commit

This is an example commit without any duplicate trailers

Co-authored-by: Billie Thompson <email@example.com>
Co-authored-by: Billie Thompson <email@example.com>
";
        test_lint_duplicated_trailers(
            message.into(),
            &Some(Problem::new(
                ERROR.into(),
                "These are normally added accidentally when you\'re rebasing or amending to a \
                 commit, sometimes in the text editor, but often by git hooks.\n\nYou can fix \
                 this by deleting the duplicated \"Co-authored-by\" field"
                    .into(),
                Code::DuplicatedTrailers,
                &message.into(),
                Some(vec![("Duplicated `Co-authored-by`".to_string(), 129, 51)]),
                None,
            )),
        );
    }

    #[test]
    fn relates_to() {
        let message = "An example commit

This is an example commit without any duplicate trailers

Relates-to: #315
Relates-to: #315
";
        test_lint_duplicated_trailers(
            message.into(),
            &Some(Problem::new(
                ERROR.into(),
                "These are normally added accidentally when you\'re rebasing or amending to a \
                 commit, sometimes in the text editor, but often by git hooks.\n\nYou can fix \
                 this by deleting the duplicated \"Relates-to\" field"
                    .into(),
                Code::DuplicatedTrailers,
                &message.into(),
                Some(vec![("Duplicated `Relates-to`".to_string(), 94, 16)]),
                None,
            )),
        );
    }

    #[test]
    fn trailer_like_duplicates_in_the_scissors_section() {
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
            &None,
        );
    }

    #[test]
    fn other_trailers() {
        test_lint_duplicated_trailers(
            "An example commit

This is an example commit without any duplicate trailers

Anything: Billie Thompson <email@example.com>
Anything: Billie Thompson <email@example.com>
"
            .into(),
            &None,
        );
    }

    fn test_lint_duplicated_trailers(message: String, expected: &Option<Problem>) {
        let actual = &lint(&CommitMessage::from(message));
        assert_eq!(
            actual, expected,
            "Expected {:?}, found {:?}",
            expected, actual
        );
    }

    #[test]
    fn formatting() {
        let message = "An example commit

This is an example commit without any duplicate trailers

Signed-off-by: Billie Thompson <email@example.com>
Signed-off-by: Billie Thompson <email@example.com>
Co-authored-by: Billie Thompson <email@example.com>
Co-authored-by: Billie Thompson <email@example.com>
";
        let problem = lint(&CommitMessage::from(message.to_string()));
        let actual = fmt_report(&Report::new(problem.unwrap()));
        let expected = "DuplicatedTrailers

  \u{d7} Your commit message has duplicated trailers
   \u{256d}\u{2500}[5:1]
 5 \u{2502} Signed-off-by: Billie Thompson <email@example.com>
 6 \u{2502} Signed-off-by: Billie Thompson <email@example.com>
   \u{b7} \u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{252c}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}
   \u{b7}                          \u{2570}\u{2500}\u{2500} Duplicated `Signed-off-by`
 7 \u{2502} Co-authored-by: Billie Thompson <email@example.com>
 8 \u{2502} Co-authored-by: Billie Thompson <email@example.com>
   \u{b7} \u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{252c}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}
   \u{b7}                          \u{2570}\u{2500}\u{2500} Duplicated `Co-authored-by`
   \u{2570}\u{2500}\u{2500}\u{2500}\u{2500}
  help: These are normally added accidentally when you're rebasing or
        amending to a commit, sometimes in the text editor, but often by
        git hooks.
        
        You can fix this by deleting the duplicated \"Co-authored-by\",
        \"Signed-off-by\" fields
"
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
}
