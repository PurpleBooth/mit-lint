use mit_commit::CommitMessage;
use std::collections::HashSet;

use crate::model::{Code, Problem, ProblemBuilder};

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

/// Configuration for conventional commit linting
#[derive(Default)]
pub struct ConventionalCommitConfig {
    /// Allowed commit types (None means any alphanumeric type is allowed)
    pub allowed_types: Option<HashSet<String>>,
    /// Allowed commit scopes (None means any word character is allowed)
    pub allowed_scopes: Option<HashSet<String>>,
}

impl ConventionalCommitConfig {
    /// Create a new configuration with custom allowed types and scopes
    ///
    /// # Arguments
    ///
    /// * `allowed_types` - Optional set of allowed commit types (None means any alphanumeric type is allowed)
    /// * `allowed_scopes` - Optional set of allowed commit scopes (None means any word character is allowed)
    ///
    /// # Returns
    ///
    /// A new `ConventionalCommitConfig` with the specified allowed types and scopes
    #[allow(dead_code)]
    pub const fn new(
        allowed_types: Option<HashSet<String>>,
        allowed_scopes: Option<HashSet<String>>,
    ) -> Self {
        Self {
            allowed_types,
            allowed_scopes,
        }
    }
}

/// Parse a conventional commit subject line
///
/// Returns (type, scope, `breaking_change`, description) if successful
fn parse_conventional_commit(subject: &str) -> Option<(String, Option<String>, bool, String)> {
    // Find the colon that separates type/scope from description
    let colon_pos = subject.find(':')?;

    // Extract the description (must have a space after the colon)
    if subject.len() <= colon_pos + 1 || subject.chars().nth(colon_pos + 1) != Some(' ') {
        return None;
    }
    let description = subject[colon_pos + 2..].to_string();

    // Parse the type, scope, and breaking change indicator
    let type_scope_part = &subject[..colon_pos];

    // Check for breaking change indicator
    let (type_scope_part, breaking_change) = type_scope_part
        .strip_suffix('!')
        .map_or((type_scope_part, false), |stripped| (stripped, true));

    // Check for scope in parentheses
    let (commit_type, scope) = if let (Some(open_paren), Some(close_paren)) =
        (type_scope_part.find('('), type_scope_part.find(')'))
    {
        if open_paren > 0 && close_paren > open_paren && close_paren == type_scope_part.len() - 1 {
            let commit_type = type_scope_part[..open_paren].to_string();
            let scope = type_scope_part[open_paren + 1..close_paren].to_string();
            (commit_type, Some(scope))
        } else {
            return None; // Malformed scope
        }
    } else {
        (type_scope_part.to_string(), None)
    };

    // Validate type is alphanumeric
    if !commit_type.chars().all(|c| c.is_ascii_alphanumeric()) || commit_type.is_empty() {
        return None;
    }

    // Validate scope is alphanumeric if present
    if let Some(scope) = &scope {
        if !scope.chars().all(char::is_alphanumeric) || scope.is_empty() {
            return None;
        }
    }

    Some((commit_type, scope, breaking_change, description))
}

/// Checks if the commit message follows the conventional commit format
///
/// # Arguments
///
/// * `commit_message` - The commit message to check
///
/// # Returns
///
/// * `Some(Problem)` - If the commit message does not follow the conventional commit format
/// * `None` - If the commit message follows the conventional commit format
///
/// # Examples
///
/// ```rust
/// use mit_commit::CommitMessage;
/// use mit_lint::Lint::NotConventionalCommit;
///
/// // This should pass
/// let passing = CommitMessage::from("feat: add new feature");
/// assert!(NotConventionalCommit.lint(&passing).is_none());
///
/// // This should fail
/// let failing = CommitMessage::from("Add new feature");
/// assert!(NotConventionalCommit.lint(&failing).is_some());
/// ```
///
/// # Errors
///
/// This function will never return an error, only an Option<Problem>
pub fn lint(commit_message: &CommitMessage<'_>) -> Option<Problem> {
    lint_with_config(commit_message, &ConventionalCommitConfig::default())
}

fn lint_with_config(
    commit_message: &CommitMessage<'_>,
    config: &ConventionalCommitConfig,
) -> Option<Problem> {
    Some(commit_message)
        .filter(|commit| has_problem(commit, config))
        .map(create_problem)
}

fn has_problem(commit_message: &CommitMessage<'_>, config: &ConventionalCommitConfig) -> bool {
    let subject: String = commit_message.get_subject().into();

    // Parse the subject line
    match parse_conventional_commit(&subject) {
        None => true, // Not a conventional commit format
        Some((commit_type, scope, _, _)) => {
            // If allowed_types is Some, check if the type is allowed
            if let Some(allowed_types) = &config.allowed_types {
                if !allowed_types.contains(&commit_type) {
                    return true;
                }
            }

            // If allowed_scopes is Some and a scope is present, check if the scope is allowed
            if let Some(allowed_scopes) = &config.allowed_scopes {
                if let Some(scope) = scope {
                    if !allowed_scopes.contains(&scope) {
                        return true;
                    }
                }
            }

            false
        }
    }
}

fn create_problem(commit_message: &CommitMessage) -> Problem {
    // Create a problem with appropriate labels
    let commit_text = String::from(commit_message.clone());
    let first_line_length = commit_text.lines().next().map(str::len).unwrap_or_default();

    ProblemBuilder::new(
        ERROR,
        HELP_MESSAGE,
        Code::NotConventionalCommit,
        commit_message,
    )
    .with_label("Not conventional", 0, first_line_length)
    .with_url("https://www.conventionalcommits.org/")
    .build()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::model::Code;
    use mit_commit::Trailer;
    use quickcheck::TestResult;

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

    #[allow(clippy::needless_pass_by_value)]
    #[quickcheck]
    fn fail_check(commit: String) -> TestResult {
        let has_non_alpha_type = commit
            .chars()
            .position(|x| x == ':')
            .is_some_and(|x| commit.chars().take(x).any(|x| !x.is_ascii_alphanumeric()));
        if has_non_alpha_type {
            return TestResult::discard();
        }
        let message = CommitMessage::from(format!("{commit}\n# comment"));
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
            || type_slug.chars().any(|x| !x.is_ascii_alphanumeric())
        {
            return TestResult::discard();
        }
        if let Some(scope) = scope.clone() {
            if scope.is_empty() || scope.chars().any(|x| !x.is_alphanumeric()) {
                return TestResult::discard();
            }
        }

        let mut commit: CommitMessage<'_> = CommitMessage::default().with_subject(
            format!(
                "{}{}{}: {}",
                type_slug,
                scope.map(|x| format!("({x})")).unwrap_or_default(),
                bc_break
                    .clone()
                    .map(|_| "!".to_string())
                    .unwrap_or_default(),
                description
            )
            .into(),
        );

        let body_contents = body.clone().unwrap_or_default();

        if body.is_some() {
            commit = commit.with_body_contents(&body_contents);
        }

        if let Some(_bc_contents) = bc_break {
            commit = commit.add_trailer(Trailer::new("BC BREAK".into(), "bc_contents".into()));
        }

        let result = lint(&commit);
        TestResult::from_bool(result.is_none())
    }
}
