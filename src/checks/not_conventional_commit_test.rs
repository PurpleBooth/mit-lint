use std::option::Option::None;

use miette::{GraphicalReportHandler, GraphicalTheme, Report};
use mit_commit::{CommitMessage, Trailer};
use quickcheck::TestResult;

use super::not_conventional_commit::{lint, ERROR, HELP_MESSAGE};
use crate::{model::Code, Problem};

// Examples from https://www.conventionalcommits.org/en/v1.0.0/

#[test]
fn commit_message_with_description_and_breaking_change_footer() {
    test_subject_not_separate_from_body(
        "feat: allow provided config object to extend other configs

BREAKING CHANGE: `extends` key in config file is now used for extending other \
 config files
",
        &None,
    );
}

#[test]
fn commit_message_with_bang_to_draw_attention_to_breaking_change() {
    test_subject_not_separate_from_body(
        "refactor!: drop support for Node 6
",
        &None,
    );
}

#[test]
fn commit_message_with_both_bang_and_breaking_change_footer() {
    test_subject_not_separate_from_body(
        "refactor!: drop support for Node 6

BREAKING CHANGE: refactor to use JavaScript features not available in Node 6.
",
        &None,
    );
}

#[test]
fn commit_message_with_no_body() {
    test_subject_not_separate_from_body(
        "docs: correct spelling of CHANGELOG
",
        &None,
    );
}

#[test]
fn commit_message_with_scope() {
    test_subject_not_separate_from_body(
        "feat(lang): add polish language
",
        &None,
    );
}

#[test]
fn commit_message_with_multi_paragraph_body_and_multiple_footers() {
    test_subject_not_separate_from_body(
        "fix: correct minor typos in code

see the issue for details

on typos fixed.

Reviewed-by: Z
Refs #133
",
        &None,
    );
}

#[test]
fn revert_example() {
    test_subject_not_separate_from_body(
        "revert: let us never again speak of the noodle incident

Refs: 676104e, a215868
",
        &None,
    );
}

#[test]
fn non_conventional() {
    let message = "An example commit

This is an example commit
";
    test_subject_not_separate_from_body(
        message,
        &Some(Problem::new(
            ERROR.into(),
            HELP_MESSAGE.into(),
            Code::NotConventionalCommit,
            &message.into(),
            Some(vec![("Not conventional".to_string(), 0_usize, 17_usize)]),
            Some("https://www.conventionalcommits.org/".parse().unwrap()),
        )),
    );
}

#[test]
fn missing_bracket() {
    let message = "fix(example: An example commit

This is an example commit
";
    test_subject_not_separate_from_body(
        message,
        &Some(Problem::new(
            ERROR.into(),
            HELP_MESSAGE.into(),
            Code::NotConventionalCommit,
            &message.into(),
            Some(vec![("Not conventional".to_string(), 0_usize, 30_usize)]),
            Some("https://www.conventionalcommits.org/".parse().unwrap()),
        )),
    );
}

#[test]
fn missing_space() {
    let message = "fix(example):An example commit

This is an example commit
";
    test_subject_not_separate_from_body(
        message,
        &Some(Problem::new(
            ERROR.into(),
            HELP_MESSAGE.into(),
            Code::NotConventionalCommit,
            &message.into(),
            Some(vec![("Not conventional".to_string(), 0_usize, 30_usize)]),
            Some("https://www.conventionalcommits.org/".parse().unwrap()),
        )),
    );
}

fn test_subject_not_separate_from_body(message: &str, expected: &Option<Problem>) {
    let actual = &lint(&CommitMessage::from(message));
    assert_eq!(
        actual, expected,
        "Message {:?} should have returned {:?}, found {:?}",
        message, expected, actual
    );
}

#[test]
fn formatting() {
    let message = "An example commit

This is an example commit
";
    let problem = lint(&CommitMessage::from(message.to_string()));
    let actual = fmt_report(&Report::new(problem.unwrap()));
    let expected = "NotConventionalCommit (https://www.conventionalcommits.org/)

  x Your commit message isn't in conventional style
   ,-[1:1]
 1 | An example commit
   : ^^^^^^^^|^^^^^^^^
   :         `-- Not conventional
 2 | 
   `----
  help: It's important to follow the conventional commit style when creating
        your commit message. By using this style we can automatically
        calculate the version of software using deployment pipelines, and
        also generate changelogs and other useful information without human
        interaction.
        
        You can fix it by following style
        
        <type>[optional scope]: <description>
        
        [optional body]
        
        [optional footer(s)]
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
fn fail_check(commit: String) -> TestResult {
    let has_non_alpha_type = commit.chars().position(|x| x == ':').map_or(false, |x| {
        commit.chars().take(x).any(|x| !x.is_ascii_alphanumeric())
    });
    if has_non_alpha_type {
        return TestResult::discard();
    }
    let message = CommitMessage::from(format!("{}\n# comment", commit));
    let result = lint(&message);
    TestResult::from_bool(result.is_some())
}

#[allow(clippy::needless_pass_by_value)]
#[quickcheck]
fn success_check(
    type_slug: String,
    scope: Option<String>,
    description: String,
    body: Option<String>,
    bc_break: Option<String>,
) -> TestResult {
    if type_slug.starts_with('#')
        || type_slug.is_empty()
        || type_slug
            .chars()
            .any(|x| x.is_whitespace() || x == '(' || x == ')')
    {
        return TestResult::discard();
    }
    if let Some(scope) = scope.clone() {
        if scope.is_empty() || scope.chars().any(|x| !x.is_alphanumeric()) {
            return TestResult::discard();
        }
    }

    let mut commit = CommitMessage::default().with_subject(&format!(
        "{}{}{}: {}",
        type_slug,
        scope.map(|x| format!("({})", x)).unwrap_or_default(),
        bc_break
            .clone()
            .map(|_| "!".to_string())
            .unwrap_or_default(),
        description
    ));

    if let Some(body_contents) = body {
        commit = commit.with_body_contents(&body_contents);
    }

    if let Some(_bc_contents) = bc_break {
        commit = commit.add_trailer(Trailer::new("BC BREAK", "bc_contents"));
    }

    let result = lint(&commit);
    TestResult::from_bool(result.is_none())
}
