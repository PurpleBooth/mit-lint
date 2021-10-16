use std::option::Option::None;

use miette::{GraphicalReportHandler, GraphicalTheme, Report};
use mit_commit::CommitMessage;
use quickcheck::TestResult;

use super::duplicate_trailers::{lint, ERROR};
use crate::{model::Code, Problem};

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
            Some(
                "https://git-scm.com/docs/githooks#_commit_msg"
                    .parse()
                    .unwrap(),
            ),
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
            Some(
                "https://git-scm.com/docs/githooks#_commit_msg"
                    .parse()
                    .unwrap(),
            ),
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
            Some("https://git-scm.com/docs/githooks#_commit_msg".to_string()),
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
            Some("https://git-scm.com/docs/githooks#_commit_msg".to_string()),
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
    let expected = "DuplicatedTrailers (https://git-scm.com/docs/githooks#_commit_msg)

  x Your commit message has duplicated trailers
   ,-[5:1]
 5 | Signed-off-by: Billie Thompson <email@example.com>
 6 | Signed-off-by: Billie Thompson <email@example.com>
   : ^^^^^^^^^^^^^^^^^^^^^^^^^|^^^^^^^^^^^^^^^^^^^^^^^^
   :                          `-- Duplicated `Signed-off-by`
 7 | Co-authored-by: Billie Thompson <email@example.com>
 8 | Co-authored-by: Billie Thompson <email@example.com>
   : ^^^^^^^^^^^^^^^^^^^^^^^^^|^^^^^^^^^^^^^^^^^^^^^^^^^
   :                          `-- Duplicated `Co-authored-by`
   `----
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
    GraphicalReportHandler::new_themed(GraphicalTheme::none())
        .with_width(80)
        .with_links(false)
        .render_report(&mut out, diag.as_ref())
        .unwrap();
    out
}

#[allow(clippy::needless_pass_by_value)]
#[quickcheck]
fn fail_check(
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
        format!("{}: {}\n", trailer_tag, trailer_text).repeat(repeats.saturating_add(2))
    ));
    let result = lint(&message);
    TestResult::from_bool(result.is_some())
}

#[allow(clippy::needless_pass_by_value)]
#[quickcheck]
fn success_check(commit: String) -> TestResult {
    let message = CommitMessage::from(commit);
    let result = lint(&message);
    TestResult::from_bool(result.is_none())
}
