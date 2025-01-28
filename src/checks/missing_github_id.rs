use std::{ops::Add, option::Option::None};

use mit_commit::CommitMessage;

use crate::model::{Code, Problem};

/// Canonical lint ID
pub const CONFIG: &str = "github-id-missing";

/// Advice on how to correct the problem
pub const HELP_MESSAGE: &str = "It's important to add the issue ID because it allows us to link code back to the motivations for doing it, and because we can help people exploring the repository link their issues to specific bits of code.

You can fix this by adding a ID like the following examples:

#642
GH-642
AnUser/git-mit#642
AnOrganisation/git-mit#642
fixes #642

Be careful just putting '#642' on a line by itself, as '#' is the default comment character" ;
/// Description of the problem
pub const ERROR: &str = "Your commit message is missing a GitHub ID";

lazy_static! {
    static ref RE: regex::Regex =
        regex::Regex::new(r"(?m)(^| )([a-zA-Z0-9_-]{3,39}/[a-zA-Z0-9-]+#|GH-|#)[0-9]+( |$)")
            .unwrap();
}

pub fn lint(commit_message: &CommitMessage<'_>) -> Option<Problem> {
    if commit_message.matches_pattern(&RE) {
        None
    } else {
        let commit_text = String::from(commit_message.clone());
        let last_line_location = commit_text
            .trim_end()
            .rfind('\n')
            .unwrap_or_default()
            .add(1);
        Some(Problem::new(
            ERROR.into(),
            HELP_MESSAGE.into(),
            Code::GitHubIdMissing,
            commit_message,
            Some(vec![(
                "No GitHub ID".to_string(),
                last_line_location,
                commit_text.len().saturating_sub(last_line_location+1),
            )]),
            Some("https://docs.github.com/en/github/writing-on-github/working-with-advanced-formatting/autolinked-references-and-urls#issues-and-pull-requests".to_string()),
        ))
    }
}

#[cfg(test)]
mod tests_has_missing_github_id {
    #![allow(clippy::wildcard_imports)]

    use std::option::Option::None;

    use miette::{GraphicalReportHandler, GraphicalTheme, Report};

    use super::*;
    use crate::model::{Code, Problem};

    #[test]
    fn id_and_close() {
        test_has_missing_github_id(
            "An example commit

This is an example commit

close #642
",
            None,
        );
        test_has_missing_github_id(
            "An example commit

This is an example commit

closes: #642
",
            None,
        );
        test_has_missing_github_id(
            "An example commit

This is an example commit

Closed GH-642
",
            None,
        );
    }

    #[test]
    fn id_and_fix() {
        test_has_missing_github_id(
            "An example commit

This is an example commit

fix #642
",
            None,
        );
        test_has_missing_github_id(
            "An example commit

This is an example commit

This fixes #642
",
            None,
        );
        test_has_missing_github_id(
            "An example commit

This is an example commit

fixed #642
",
            None,
        );
    }

    #[test]
    fn id_and_resolve() {
        test_has_missing_github_id(
            "An example commit

This is an example commit

fixed #642
",
            None,
        );
        test_has_missing_github_id(
            "An example commit

This is an example commit

resolve #642
",
            None,
        );
        test_has_missing_github_id(
            "An example commit

This is an example commit

resolves #642
",
            None,
        );
    }

    #[test]
    fn issue() {
        test_has_missing_github_id(
            "An example commit

This is an example commit

resolved #642
",
            None,
        );
        test_has_missing_github_id(
            "An example commit

This is an example commit

Issue #642
",
            None,
        );
    }

    #[test]
    fn gh_id_variant() {
        test_has_missing_github_id(
            "An example commit

This is an example commit

GH-642
",
            None,
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
            None,
        );
    }

    #[test]
    fn long_variant() {
        test_has_missing_github_id(
            "An example commit

This is an example commit

AnUser/git-mit#642
",
            None,
        );

        test_has_missing_github_id(
            "An example commit

This is an example commit

AnOrganisation/git-mit#642
",
            None,
        );
    }

    #[test]
    fn id_missing() {
        let message = "An example commit

This is an example commit
";
        test_has_missing_github_id(
            message,
            Some(Problem::new(
                ERROR.into(),
                HELP_MESSAGE.into(),
                Code::GitHubIdMissing,
                &message.into(),
                Some(vec![(String::from("No GitHub ID"), 19, 25)]),
                Some(String::from("https://docs.github.com/en/github/writing-on-github/working-with-advanced-formatting/autolinked-references-and-urls#issues-and-pull-requests")),
            )).as_ref(),
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
            Some(Problem::new(
                ERROR.into(),
                HELP_MESSAGE.into(),
                Code::GitHubIdMissing,
                &message_1.into(),
                Some(vec![("No GitHub ID".to_string(), 46, 5)]),
                Some("https://docs.github.com/en/github/writing-on-github/working-with-advanced-formatting/autolinked-references-and-urls#issues-and-pull-requests".parse().unwrap()),
            )).as_ref(),
        );
        let message_2 = "An example commit

This is an example commit

git-mit#123
";
        test_has_missing_github_id(
            message_2,
            Some(Problem::new(
                ERROR.into(),
                HELP_MESSAGE.into(),
                Code::GitHubIdMissing,
                &message_2.into(),
                Some(vec![("No GitHub ID".to_string(), 46, 11)]),
                Some("https://docs.github.com/en/github/writing-on-github/working-with-advanced-formatting/autolinked-references-and-urls#issues-and-pull-requests".parse().unwrap()),
            )).as_ref(),
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
   ,-[3:1]
 2 | 
 3 | This is an example commit
   : ^^^^^^^^^^^^|^^^^^^^^^^^^
   :             `-- No GitHub ID
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

    fn test_has_missing_github_id(message: &str, expected: Option<&Problem>) {
        let actual = &lint(&CommitMessage::from(message));
        assert_eq!(
            actual.as_ref(),
            expected,
            "Message {message:?} should have returned {expected:?}, found {actual:?}"
        );
    }
}
