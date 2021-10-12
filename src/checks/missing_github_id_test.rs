use std::option::Option::None;

use miette::{GraphicalReportHandler, GraphicalTheme, Report};
use mit_commit::CommitMessage;
use quickcheck::TestResult;

use super::missing_github_id::{lint, ERROR, HELP_MESSAGE};
use crate::model::{Code, Problem};

#[test]
fn id_and_close() {
    test_has_missing_github_id(
        "An example commit

This is an example commit

close #642
",
        &None,
    );
    test_has_missing_github_id(
        "An example commit

This is an example commit

closes: #642
",
        &None,
    );
    test_has_missing_github_id(
        "An example commit

This is an example commit

Closed GH-642
",
        &None,
    );
}

#[test]
fn id_and_fix() {
    test_has_missing_github_id(
        "An example commit

This is an example commit

fix #642
",
        &None,
    );
    test_has_missing_github_id(
        "An example commit

This is an example commit

This fixes #642
",
        &None,
    );
    test_has_missing_github_id(
        "An example commit

This is an example commit

fixed #642
",
        &None,
    );
}

#[test]
fn id_and_resolve() {
    test_has_missing_github_id(
        "An example commit

This is an example commit

fixed #642
",
        &None,
    );
    test_has_missing_github_id(
        "An example commit

This is an example commit

resolve #642
",
        &None,
    );
    test_has_missing_github_id(
        "An example commit

This is an example commit

resolves #642
",
        &None,
    );
}

#[test]
fn issue() {
    test_has_missing_github_id(
        "An example commit

This is an example commit

resolved #642
",
        &None,
    );
    test_has_missing_github_id(
        "An example commit

This is an example commit

Issue #642
",
        &None,
    );
}

#[test]
fn gh_id_variant() {
    test_has_missing_github_id(
        "An example commit

This is an example commit

GH-642
",
        &None,
    );
}

#[test]
fn hash_alone_variant() {
    test_has_missing_github_id(
        "An example commit

This is an example commit

#642
; Comment character is set to something else like ';'
",
        &None,
    );
}

#[test]
fn long_variant() {
    test_has_missing_github_id(
        "An example commit

This is an example commit

AnUser/git-mit#642
",
        &None,
    );

    test_has_missing_github_id(
        "An example commit

This is an example commit

AnOrganisation/git-mit#642
",
        &None,
    );
}

#[test]
fn id_missing() {
    let message = "An example commit

This is an example commit
";
    test_has_missing_github_id(
            message,
            &Some(Problem::new(
                ERROR.into(),
                HELP_MESSAGE.into(),
                Code::GitHubIdMissing,
                &message.into(),
                Some(vec![(String::from("No GitHub ID"), 19, 26)]),
                Some(String::from("https://docs.github.com/en/github/writing-on-github/working-with-advanced-formatting/autolinked-references-and-urls#issues-and-pull-requests")),
            )),
        );
}

#[test]
fn id_malformed() {
    let message_1 = "An example commit

This is an example commit

H-123
";
    test_has_missing_github_id(
            message_1,
            &Some(Problem::new(
                ERROR.into(),
                HELP_MESSAGE.into(),
                Code::GitHubIdMissing,
                &message_1.into(),
                Some(vec![("No GitHub ID".to_string(), 46, 6)]),
                Some("https://docs.github.com/en/github/writing-on-github/working-with-advanced-formatting/autolinked-references-and-urls#issues-and-pull-requests".parse().unwrap()),
            )),
        );
    let message_2 = "An example commit

This is an example commit

git-mit#123
";
    test_has_missing_github_id(
            message_2,
            &Some(Problem::new(
                ERROR.into(),
                HELP_MESSAGE.into(),
                Code::GitHubIdMissing,
                &message_2.into(),
                Some(vec![("No GitHub ID".to_string(), 46, 12)]),
                Some("https://docs.github.com/en/github/writing-on-github/working-with-advanced-formatting/autolinked-references-and-urls#issues-and-pull-requests".parse().unwrap()),
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
    let expected = "GitHubIdMissing (https://docs.github.com/en/github/writing-on-github/working-with-advanced-formatting/autolinked-references-and-urls#issues-and-pull-requests)

  x Your commit message is missing a GitHub ID
   ,-[2:1]
 2 | 
 3 | This is an example commit
   : ^^^^^^^^^^^^^|^^^^^^^^^^^^
   :              `-- No GitHub ID
   `----
  help: It's important to add the issue ID because it allows us to link code
        back to the motivations for doing it, and because we can help people
        exploring the repository link their issues to specific bits of code.
        
        You can fix this by adding a ID like the following examples:
        
        #642
        GH-642
        AnUser/git-mit#642
        AnOrganisation/git-mit#642
        fixes #642
        
        Be careful just putting '#642' on a line by itself, as '#' is the
        default comment character
".to_string();
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

fn test_has_missing_github_id(message: &str, expected: &Option<Problem>) {
    let actual = &lint(&CommitMessage::from(message));
    assert_eq!(
        actual, expected,
        "Message {:?} should have returned {:?}, found {:?}",
        message, expected, actual
    );
}

#[allow(clippy::needless_pass_by_value)]
#[quickcheck]
fn fail_check(commit: String) -> TestResult {
    if commit == "\u{0}: " {
        return TestResult::discard();
    }

    let message = CommitMessage::from(commit);
    let result = lint(&message);
    TestResult::from_bool(result.is_some())
}

#[allow(clippy::needless_pass_by_value)]
#[quickcheck]
fn success_with_gh_key_check(
    commit: Option<String>,
    commit_suffix: Option<String>,
    id: usize,
) -> TestResult {
    if commit
        .clone()
        .filter(|x| x.starts_with('#') || x.contains("\n#"))
        .is_some()
    {
        return TestResult::discard();
    }

    let message = CommitMessage::from(format!(
        "{}GH-{}{}\n# comment",
        commit.map(|x| format!("{} ", x)).unwrap_or_default(),
        id,
        commit_suffix.map(|x| format!(" {}", x)).unwrap_or_default()
    ));
    let result = lint(&message);
    TestResult::from_bool(result.is_none())
}

#[allow(clippy::needless_pass_by_value)]
#[quickcheck]
fn success_with_id_key_check(
    commit: Option<String>,
    commit_suffix: Option<String>,
    id: usize,
) -> TestResult {
    if let Some(ref initial) = commit {
        if initial.starts_with('!') || initial.contains("\n!") {
            return TestResult::discard();
        }
    }

    let message = CommitMessage::from(format!(
        "{}#{}{}\n! comment",
        commit.map(|x| format!("{} ", x)).unwrap_or_default(),
        id,
        commit_suffix.map(|x| format!(" {}", x)).unwrap_or_default()
    ));
    let result = lint(&message);
    TestResult::from_bool(result.is_none())
}

#[allow(clippy::needless_pass_by_value)]
#[quickcheck]
fn success_with_with_org_id_key_check(
    commit: Option<String>,
    commit_suffix: Option<String>,
    org: String,
    repo: String,
    id: usize,
) -> TestResult {
    if commit.clone().filter(|x| x.starts_with('#')).is_some() {
        return TestResult::discard();
    }

    if org.is_empty() || org.chars().count() < 3 || org.chars().any(|x| !x.is_ascii_alphanumeric())
    {
        return TestResult::discard();
    }
    if repo.is_empty() || repo.chars().any(|x| !x.is_ascii_alphanumeric()) {
        return TestResult::discard();
    }

    let message = CommitMessage::from(format!(
        "{}{}/{}#{}{}",
        commit.map(|x| format!("{} ", x)).unwrap_or_default(),
        org,
        repo,
        id,
        commit_suffix.map(|x| format!(" {}", x)).unwrap_or_default()
    ));
    let result = lint(&message);
    TestResult::from_bool(result.is_none())
}
