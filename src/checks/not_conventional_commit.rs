use std::{option::Option::None, sync::LazyLock};

use mit_commit::CommitMessage;
use regex::Regex;

use crate::model::{Code, Problem};

/// Canonical lint ID
pub const CONFIG: &str = "not-conventional-commit";

/// Advice on how to correct the problem
pub const HELP_MESSAGE: &str =
    "It's important to follow the conventional commit style when creating your commit message. By \
using this style we can automatically calculate the version of software using deployment \
pipelines, and also generate changelogs and other useful information without human interaction.

You can fix it by following style

<type>[optional scope]: <description>

[optional body]

[optional footer(s)]";
/// Description of the problem
pub const ERROR: &str = "Your commit message isn't in conventional style";

static RE: LazyLock<Regex> = LazyLock::new(|| Regex::new("^[a-zA-Z0-9]+(\\(\\w+\\))?!?: ").unwrap());

fn has_problem(commit_message: &CommitMessage<'_>) -> bool {
    let subject: String = commit_message.get_subject().into();

    !RE.is_match(&subject)
}

pub fn lint(commit_message: &CommitMessage<'_>) -> Option<Problem> {
    if has_problem(commit_message) {
        let commit_text = String::from(commit_message.clone());
        Some(Problem::new(
            ERROR.into(),
            HELP_MESSAGE.into(),
            Code::NotConventionalCommit,
            commit_message,
            Some(vec![(
                "Not conventional".to_string(),
                0_usize,
                commit_text.lines().next().map(str::len).unwrap_or_default(),
            )]),
            Some("https://www.conventionalcommits.org/".to_string()),
        ))
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    #![allow(clippy::wildcard_imports)]

    use super::*;
    use crate::model::Code;

    // Examples from https://www.conventionalcommits.org/en/v1.0.0/

    #[test]
    fn commit_message_with_description_and_breaking_change_footer() {
        test_subject_not_separate_from_body(
            "feat: allow provided config object to extend other configs

BREAKING CHANGE: `extends` key in config file is now used for extending other \
 config files
",
            None,
        );
    }

    #[test]
    fn commit_message_with_bang_to_draw_attention_to_breaking_change() {
        test_subject_not_separate_from_body(
            "refactor!: drop support for Node 6
",
            None,
        );
    }

    #[test]
    fn commit_message_with_both_bang_and_breaking_change_footer() {
        test_subject_not_separate_from_body(
            "refactor!: drop support for Node 6

BREAKING CHANGE: refactor to use JavaScript features not available in Node 6.
",
            None,
        );
    }

    #[test]
    fn commit_message_with_no_body() {
        test_subject_not_separate_from_body(
            "docs: correct spelling of CHANGELOG
",
            None,
        );
    }

    #[test]
    fn commit_message_with_scope() {
        test_subject_not_separate_from_body(
            "feat(lang): add polish language
",
            None,
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
            None,
        );
    }

    #[test]
    fn revert_example() {
        test_subject_not_separate_from_body(
            "revert: let us never again speak of the noodle incident

Refs: 676104e, a215868
",
            None,
        );
    }

    #[test]
    fn non_conventional() {
        let message = "An example commit

This is an example commit
";
        test_subject_not_separate_from_body(
            message,
            Some(&Problem::new(
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
            Some(Problem::new(
                ERROR.into(),
                HELP_MESSAGE.into(),
                Code::NotConventionalCommit,
                &message.into(),
                Some(vec![("Not conventional".to_string(), 0_usize, 30_usize)]),
                Some("https://www.conventionalcommits.org/".parse().unwrap()),
            ))
            .as_ref(),
        );
    }

    #[test]
    fn missing_space() {
        let message = "fix(example):An example commit

This is an example commit
";
        test_subject_not_separate_from_body(
            message,
            Some(Problem::new(
                ERROR.into(),
                HELP_MESSAGE.into(),
                Code::NotConventionalCommit,
                &message.into(),
                Some(vec![("Not conventional".to_string(), 0_usize, 30_usize)]),
                Some("https://www.conventionalcommits.org/".parse().unwrap()),
            ))
            .as_ref(),
        );
    }

    fn test_subject_not_separate_from_body(message: &str, expected: Option<&Problem>) {
        let actual = &lint(&CommitMessage::from(message));
        assert_eq!(
            actual.as_ref(),
            expected,
            "Message {message:?} should have returned {expected:?}, found {actual:?}"
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
        calculate the version of software using deployment pipelines, and also
        generate changelogs and other useful information without human
        interaction.
        
        You can fix it by following style
        
        <type>[optional scope]: <description>
        
        [optional body]
        
        [optional footer(s)]
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
}
