use std::sync::LazyLock;

use mit_commit::CommitMessage;
use regex::Regex;

use crate::model::{Code, Problem, ProblemBuilder};

/// Canonical lint ID
pub const CONFIG: &str = "gitlab-id-missing";

/// Advice on how to correct the problem
pub const HELP_MESSAGE: &str = "It's important to add the issue ID because it allows us to link code back to the motivations for doing it, and because we can help people exploring the repository link their issues to specific bits of code.

You can fix this by adding an ID like the following examples:

#642
GL-642
AnUser/git-mit#642
group/subgroup/project#642
https://gitlab.com/AnUser/git-mit/-/issues/642
https://gitlab.com/group/subgroup/project/-/work_items/642
fixes #642

Be careful just putting '#642' on a line by itself, as '#' is the default comment character";

/// Description of the problem
pub const ERROR: &str = "Your commit message is missing a GitLab ID";

static RE: LazyLock<Regex> = LazyLock::new(|| {
    // GitLab issue reference patterns:
    // 1. Same project: #123
    // 2. Alternative: GL-123
    // 3. Cross-project (supports nested groups): group/subgroup/project#123
    // 4. Full URLs: https://gitlab.com/user/project/-/issues/123 or .../-/work_items/123
    Regex::new(
        r"(?m)(^| |\()((?:[a-zA-Z0-9_-]+/)+[a-zA-Z0-9_-]+#|GL-|#)[0-9]+\b|https?://\S+/-/(?:issues|work_items)/[0-9]+\b"
    ).unwrap()
});

pub struct GitLabIdConfig {
    /// Regular expression for matching GitLab IDs
    pub pattern: Regex,
}

impl Default for GitLabIdConfig {
    fn default() -> Self {
        Self {
            pattern: RE.clone(),
        }
    }
}

/// Checks if the commit message contains a GitLab ID
///
/// # Arguments
///
/// * `commit_message` - The commit message to check
///
/// # Returns
///
/// * `Some(Problem)` - If the commit message does not contain a GitLab ID
/// * `None` - If the commit message contains a GitLab ID
///
/// # Examples
///
/// ```rust
/// use mit_commit::CommitMessage;
/// use mit_lint::Lint;
/// use mit_lint::Lint::GitLabIdMissing;
///
/// // This should pass
/// let passing = CommitMessage::from("Subject\n\nBody\n\nRelates-to: #123");
/// assert!(GitLabIdMissing.lint(&passing).is_none());
///
/// // This should fail
/// let failing = CommitMessage::from("Subject\n\nBody");
/// assert!(GitLabIdMissing.lint(&failing).is_some());
/// ```
///
/// # Errors
///
/// This function will never return an error, only an Option<Problem>
pub fn lint(commit_message: &CommitMessage<'_>) -> Option<Problem> {
    lint_with_config(commit_message, &GitLabIdConfig::default())
}

fn lint_with_config(
    commit_message: &CommitMessage<'_>,
    config: &GitLabIdConfig,
) -> Option<Problem> {
    Some(commit_message)
        .filter(|commit| has_problem(commit, &config.pattern))
        .map(create_problem)
}

fn has_problem(commit_message: &CommitMessage<'_>, pattern: &Regex) -> bool {
    !commit_message.matches_pattern(pattern)
}

fn create_problem(commit_message: &CommitMessage) -> Problem {
    ProblemBuilder::new(ERROR, HELP_MESSAGE, Code::GitLabIdMissing, commit_message)
        .with_label_at_last_line("No GitLab ID")
        .with_url("https://docs.gitlab.com/ee/user/markdown.html#gitlab-specific-references")
        .build()
}

#[cfg(test)]
mod tests {
    use std::option::Option::None;

    use miette::{GraphicalReportHandler, GraphicalTheme, Report};
    use mit_commit::CommitMessage;
    use quickcheck::TestResult;

    use super::*;
    use crate::model::{Code, Problem};

    #[test]
    fn test_gitlab_id_with_close_keyword_passes() {
        test_has_missing_gitlab_id(
            "An example commit

This is an example commit

close #642
",
            None,
        );
        test_has_missing_gitlab_id(
            "An example commit

This is an example commit

closes: #642
",
            None,
        );
        test_has_missing_gitlab_id(
            "An example commit

This is an example commit

Closed GL-642
",
            None,
        );
    }

    #[test]
    fn test_gitlab_id_with_fix_keyword_passes() {
        test_has_missing_gitlab_id(
            "An example commit

This is an example commit

fix #642
",
            None,
        );
        test_has_missing_gitlab_id(
            "An example commit

This is an example commit

This fixes #642
",
            None,
        );
        test_has_missing_gitlab_id(
            "An example commit

This is an example commit

fixed #642
",
            None,
        );
    }

    #[test]
    fn test_gitlab_id_with_resolve_keyword_passes() {
        test_has_missing_gitlab_id(
            "An example commit

This is an example commit

fixed #642
",
            None,
        );
        test_has_missing_gitlab_id(
            "An example commit

This is an example commit

resolve #642
",
            None,
        );
        test_has_missing_gitlab_id(
            "An example commit

This is an example commit

resolves #642
",
            None,
        );
    }

    #[test]
    fn test_gitlab_id_with_issue_keyword_passes() {
        test_has_missing_gitlab_id(
            "An example commit

This is an example commit

resolved #642
",
            None,
        );
        test_has_missing_gitlab_id(
            "An example commit

This is an example commit

Issue #642
",
            None,
        );
    }

    #[test]
    fn test_gitlab_id_with_gl_prefix_passes() {
        test_has_missing_gitlab_id(
            "An example commit

This is an example commit

GL-642
",
            None,
        );
    }

    #[test]
    fn test_gitlab_id_with_hash_only_passes() {
        test_has_missing_gitlab_id(
            "An example commit

This is an example commit

#642
; Comment character is set to something else like ';'
",
            None,
        );
    }

    #[test]
    fn test_gitlab_id_with_org_repo_format_passes() {
        test_has_missing_gitlab_id(
            "An example commit

This is an example commit

AnUser/git-mit#642
",
            None,
        );

        test_has_missing_gitlab_id(
            "An example commit

This is an example commit

AnOrganisation/git-mit#642
",
            None,
        );
    }

    #[test]
    fn test_gitlab_id_with_nested_groups_passes() {
        // GitLab supports nested group paths
        test_has_missing_gitlab_id(
            "An example commit

This is an example commit

group/subgroup/project#642
",
            None,
        );
    }

    #[test]
    fn test_gitlab_id_with_full_url_passes() {
        // Full URLs with /-issues/ and /-work_items/
        test_has_missing_gitlab_id(
            "An example commit

This is an example commit

https://gitlab.com/user/project/-/issues/642
",
            None,
        );
        test_has_missing_gitlab_id(
            "An example commit

This is an example commit

https://gitlab.example.com/group/subgroup/project/-/work_items/642
",
            None,
        );
    }

    #[test]
    fn test_gitlab_id_with_underscore_in_repo_name_passes() {
        // GitLab repos can have underscores in their names (e.g. my_repo)
        test_has_missing_gitlab_id(
            "An example commit

This is an example commit

AnUser/my_repo#642
",
            None,
        );
    }

    #[test]
    fn test_gitlab_id_followed_by_punctuation_passes() {
        // A GitLab ID reference followed by common punctuation — an
        // end-of-sentence period, or wrapped in parentheses — should still be
        // recognised as a valid reference and NOT flagged as missing.
        test_has_missing_gitlab_id("An example commit\n\nSee issue #642.\n", None);
        test_has_missing_gitlab_id("An example commit\n\nFixes (#642) the crash\n", None);
    }

    #[test]
    fn test_commit_without_gitlab_id_fails() {
        let message = "An example commit

This is an example commit
";
        test_has_missing_gitlab_id(
            message,
            Some(Problem::new(
                ERROR.into(),
                HELP_MESSAGE.into(),
                Code::GitLabIdMissing,
                &message.into(),
                Some(vec![(String::from("No GitLab ID"), 19, 25)]),
                Some(String::from(
                    "https://docs.gitlab.com/ee/user/markdown.html#gitlab-specific-references",
                )),
            ))
            .as_ref(),
        );
    }

    #[test]
    fn test_commit_with_malformed_gitlab_id_fails() {
        let message_1 = "An example commit

This is an example commit

G-123
";
        test_has_missing_gitlab_id(
            message_1,
            Some(Problem::new(
                ERROR.into(),
                HELP_MESSAGE.into(),
                Code::GitLabIdMissing,
                &message_1.into(),
                Some(vec![("No GitLab ID".to_string(), 46, 5)]),
                Some(
                    "https://docs.gitlab.com/ee/user/markdown.html#gitlab-specific-references"
                        .parse()
                        .unwrap(),
                ),
            ))
            .as_ref(),
        );
        let message_2 = "An example commit

This is an example commit

git-mit#123
";
        test_has_missing_gitlab_id(
            message_2,
            Some(Problem::new(
                ERROR.into(),
                HELP_MESSAGE.into(),
                Code::GitLabIdMissing,
                &message_2.into(),
                Some(vec![("No GitLab ID".to_string(), 46, 11)]),
                Some(
                    "https://docs.gitlab.com/ee/user/markdown.html#gitlab-specific-references"
                        .parse()
                        .unwrap(),
                ),
            ))
            .as_ref(),
        );
    }

    #[test]
    fn test_error_report_formatting() {
        let message = "An example commit

This is an example commit
";
        let problem = lint(&CommitMessage::from(message.to_string()));
        let actual = fmt_report(&Report::new(problem.unwrap()));
        let expected = "GitLabIdMissing (https://docs.gitlab.com/ee/user/markdown.html#gitlab-specific-references)

  x Your commit message is missing a GitLab ID
   ,-[3:1]
 2 | 
 3 | This is an example commit
   : ^^^^^^^^^^^^|^^^^^^^^^^^^
   :             `-- No GitLab ID
   `----
  help: It's important to add the issue ID because it allows us to link code
        back to the motivations for doing it, and because we can help people
        exploring the repository link their issues to specific bits of code.
        
        You can fix this by adding an ID like the following examples:
        
        #642
        GL-642
        AnUser/git-mit#642
        group/subgroup/project#642
        https://gitlab.com/AnUser/git-mit/-/issues/642
        https://gitlab.com/group/subgroup/project/-/work_items/642
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

    fn test_has_missing_gitlab_id(message: &str, expected: Option<&Problem>) {
        let actual = &lint(&CommitMessage::from(message));
        assert_eq!(
            actual.as_ref(),
            expected,
            "Message {message:?} should have returned {expected:?}, found {actual:?}"
        );
    }

    #[allow(clippy::needless_pass_by_value)]
    #[quickcheck]
    fn test_quickcheck_commits_without_gitlab_id_fail(commit: String) -> TestResult {
        if commit == "\u{0}: " {
            return TestResult::discard();
        }

        let message = CommitMessage::from(commit);

        // Discard commits that actually contain a GitLab ID. The lint returns
        // `None` for those (by design), so they are outside this property's
        // scope, which asserts that a commit *without* an ID is flagged.
        if message.matches_pattern(&GitLabIdConfig::default().pattern) {
            return TestResult::discard();
        }

        let result = lint(&message);
        TestResult::from_bool(result.is_some())
    }

    #[allow(clippy::needless_pass_by_value)]
    #[quickcheck]
    fn test_quickcheck_commits_with_gl_prefix_pass(
        commit: Option<String>,
        commit_suffix: Option<String>,
        id: usize,
    ) -> TestResult {
        if commit
            .as_ref()
            .is_some_and(|x| x.starts_with('#') || x.contains("\n#"))
        {
            return TestResult::discard();
        }

        let message = CommitMessage::from(format!(
            "{}GL-{}{}\n# comment",
            commit.map(|x| format!("{x} ")).unwrap_or_default(),
            id,
            commit_suffix.map(|x| format!(" {x}")).unwrap_or_default()
        ));
        let result = lint(&message);
        TestResult::from_bool(result.is_none())
    }

    #[allow(clippy::needless_pass_by_value)]
    #[quickcheck]
    fn test_quickcheck_commits_with_hash_id_pass(
        commit: Option<String>,
        commit_suffix: Option<String>,
        id: usize,
    ) -> TestResult {
        if let Some(ref initial) = commit
            && (initial.starts_with('!') || initial.contains("\n!"))
        {
            return TestResult::discard();
        }

        let message = CommitMessage::from(format!(
            "{}#{}{}\n! comment",
            commit.map(|x| format!("{x} ")).unwrap_or_default(),
            id,
            commit_suffix.map(|x| format!(" {x}")).unwrap_or_default()
        ));
        let result = lint(&message);
        TestResult::from_bool(result.is_none())
    }

    #[allow(clippy::needless_pass_by_value)]
    #[quickcheck]
    fn test_quickcheck_commits_with_org_repo_format_pass(
        commit: Option<String>,
        commit_suffix: Option<String>,
        org: String,
        repo: String,
        id: usize,
    ) -> TestResult {
        if commit.as_ref().is_some_and(|x| x.starts_with('#')) {
            return TestResult::discard();
        }

        if org.is_empty()
            || org.chars().count() < 3
            || org.chars().any(|x| !x.is_ascii_alphanumeric())
        {
            return TestResult::discard();
        }
        if repo.is_empty() || repo.chars().any(|x| !x.is_ascii_alphanumeric()) {
            return TestResult::discard();
        }

        let message = CommitMessage::from(format!(
            "{}{}/{}#{}{}",
            commit.map(|x| format!("{x} ")).unwrap_or_default(),
            org,
            repo,
            id,
            commit_suffix.map(|x| format!(" {x}")).unwrap_or_default()
        ));
        let result = lint(&message);
        TestResult::from_bool(result.is_none())
    }
}
