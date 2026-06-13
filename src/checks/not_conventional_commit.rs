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
    #[allow(dead_code)] // Used in tests only; public API for downstream consumers
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
    // Use byte slicing (safe because ':' is ASCII) rather than .chars().nth()
    // which would treat the byte offset as a character index and break on
    // multi-byte UTF-8 in the scope.
    if subject.len() <= colon_pos + 1 || !subject[colon_pos + 1..].starts_with(' ') {
        return None;
    }
    // Extract the description (can be empty)
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

    // Validate scope if present. Hyphens are permitted (e.g. component
    // identifiers like "git-mit"); otherwise the scope must be non-empty and
    // made up of alphanumeric characters.
    if let Some(scope) = &scope
        && (!scope.chars().all(|c| c.is_alphanumeric() || c == '-') || scope.is_empty())
    {
        return None;
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
            if let Some(allowed_types) = &config.allowed_types
                && !allowed_types.contains(&commit_type)
            {
                return true;
            }

            // If allowed_scopes is Some and a scope is present, check if the scope is allowed
            if let Some(allowed_scopes) = &config.allowed_scopes
                && let Some(scope) = scope
                && !allowed_scopes.contains(&scope)
            {
                return true;
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
        let expected = "NotConventionalCommit (https://www.conventionalcommits.org/)\n\n  x Your commit message isn't in conventional style\n   ,-[1:1]\n 1 | An example commit\n   : ^^^^^^^^|^^^^^^^^\n   :         `-- Not conventional\n 2 | \n   `----\n  help: It's important to follow the conventional commit style when creating\n        your commit message. By using this style we can automatically\n        calculate the version of software using deployment pipelines, and also\n        generate changelogs and other useful information without human\n        interaction.\n        \n        You can fix it by following style\n        \n        <type>[optional scope]: <description>\n        \n        [optional body]\n        \n        [optional footer(s)]\n"
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
        // Discard inputs that are actually valid conventional commits,
        // since the property asserts that lint reports a problem.
        // Without this guard, inputs like "feat: description" would be
        // kept, and the property would wrongly assert is_some() when the
        // lint correctly returns None.
        if parse_conventional_commit(&commit).is_some() {
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
        if let Some(scope) = scope.clone()
            && (scope.is_empty() || scope.chars().any(|x| !x.is_alphanumeric() && x != '-'))
        {
            return TestResult::discard();
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

    // Tests for custom configurations with allowed_types and allowed_scopes
    #[test]
    fn test_lint_with_config_allowed_types() {
        use std::collections::HashSet;

        // Create a config that only allows "feat" type
        let mut allowed_types = HashSet::new();
        allowed_types.insert("feat".to_string());
        let config = ConventionalCommitConfig::new(Some(allowed_types), None);

        // Test with allowed type
        let commit_allowed = CommitMessage::from("feat: add new feature");
        assert!(lint_with_config(&commit_allowed, &config).is_none());

        // Test with disallowed type
        let commit_disallowed = CommitMessage::from("fix: fix a bug");
        assert!(lint_with_config(&commit_disallowed, &config).is_some());
    }

    #[test]
    fn test_lint_with_config_allowed_scopes() {
        use std::collections::HashSet;

        // Create a config that only allows "ui" scope
        let mut allowed_scopes = HashSet::new();
        allowed_scopes.insert("ui".to_string());
        let config = ConventionalCommitConfig::new(None, Some(allowed_scopes));

        // Test with allowed scope
        let commit_allowed = CommitMessage::from("feat(ui): add new UI feature");
        assert!(lint_with_config(&commit_allowed, &config).is_none());

        // Test with disallowed scope
        let commit_disallowed = CommitMessage::from("feat(api): add new API feature");
        assert!(lint_with_config(&commit_disallowed, &config).is_some());
    }

    // Tests for edge cases in parse_conventional_commit
    #[test]
    fn test_parse_conventional_commit_colon_position() {
        // Test with no space after colon (should fail)
        assert!(parse_conventional_commit("feat:no-space").is_none());

        // Test with space after colon (should pass)
        assert!(parse_conventional_commit("feat: with-space").is_some());

        // Test with colon at the end (should fail)
        assert!(parse_conventional_commit("feat:").is_none());

        // Test with colon at the end followed by a space (should pass)
        // This specifically tests the case that failed in the quickcheck test
        assert!(parse_conventional_commit("feat: ").is_some());

        // Test with colon at position 0 (should fail because the commit type is empty)
        assert!(parse_conventional_commit(": description").is_none());

        // Test with colon at a high position (should pass if followed by space and description)
        let long_type = "a".repeat(100);
        let commit_message = format!("{long_type}(scope): description");
        assert!(parse_conventional_commit(&commit_message).is_some());
    }

    #[test]
    fn test_parse_conventional_commit_scope_parsing() {
        // Test with valid scope
        let result = parse_conventional_commit("feat(ui): add feature");
        assert!(result.is_some());
        let (commit_type, scope, _, _) = result.unwrap();
        assert_eq!(commit_type, "feat");
        assert_eq!(scope, Some("ui".to_string()));

        // Test with malformed scope (open paren at beginning)
        assert!(parse_conventional_commit("(ui): add feature").is_none());

        // Test with malformed scope (close paren not at end)
        assert!(parse_conventional_commit("feat(ui)extra: add feature").is_none());

        // Test with malformed scope (open paren after close paren)
        assert!(parse_conventional_commit("feat)(: add feature").is_none());
    }

    #[test]
    fn test_parse_conventional_commit_scope_validation() {
        // Test with empty scope (should fail)
        assert!(parse_conventional_commit("feat(): add feature").is_none());

        // Test with scope containing characters other than alphanumerics and
        // hyphens (should fail)
        assert!(parse_conventional_commit("feat(ui_component): add feature").is_none());

        // Test with hyphenated scope (should pass)
        assert!(parse_conventional_commit("feat(ui-component): add feature").is_some());

        // Test with alphanumeric scope (should pass)
        assert!(parse_conventional_commit("feat(ui123): add feature").is_some());
    }

    #[test]
    fn test_parse_conventional_commit_scope_allows_hyphens() {
        // Hyphens are common in scope names — module/component identifiers
        // such as "git-mit" — and should be accepted by the parser.
        let result = parse_conventional_commit("fix(git-mit): Some text");
        assert!(
            result.is_some(),
            "scope with a hyphen should parse successfully"
        );

        let (commit_type, scope, _, _) = result.unwrap();
        assert_eq!(commit_type, "fix");
        assert_eq!(scope.as_deref(), Some("git-mit"));

        // The full lint should also accept it.
        let commit = CommitMessage::from("fix(git-mit): Some text\n");
        assert!(
            lint(&commit).is_none(),
            "commit with a hyphenated scope should pass the lint"
        );
    }

    #[test]
    fn test_quickcheck_failing_case() {
        // Test the specific case that failed in QuickCheck: ("0", None, "", None, None)
        let commit = CommitMessage::from("0: ");
        assert!(lint(&commit).is_none());
    }

    #[test]
    fn test_parse_conventional_commit_with_multibyte_scope() {
        // Bug: parse_conventional_commit mixes byte offsets with char indices
        // when checking for space after colon. Multi-byte chars in the scope
        // cause the byte position of ':' to differ from its char index.
        //
        // "feat(ü): description" - 'ü' is 2 bytes in UTF-8, so colon is at
        // byte 9 but char index 7. Using byte position as char index checks
        // the wrong character.

        // Unicode scope with 2-byte char
        assert!(
            parse_conventional_commit("feat(ü): add feature").is_some(),
            "conventional commit with Unicode scope 'ü' should parse successfully"
        );

        // Verify full lint accepts it too
        let commit = CommitMessage::from("feat(ü): add feature\n");
        assert!(
            lint(&commit).is_none(),
            "commit with Unicode scope should pass lint"
        );
    }

    #[test]
    fn test_parse_conventional_commit_with_cjk_scope() {
        // CJK characters are 3 bytes in UTF-8, more likely to trigger the bug
        assert!(
            parse_conventional_commit("feat(機能): add feature").is_some(),
            "conventional commit with CJK scope should parse successfully"
        );
    }

    // Regression test: the fail_check quickcheck property had an inadequate
    // guard that didn't discard valid conventional commits.  Inputs like
    // "feat: description" were kept and the property wrongly asserted that
    // lint should report a problem, when in fact lint correctly returns None.
    //
    // The fix is to also discard inputs that parse as valid conventional
    // commits.
    #[test]
    fn test_fail_check_guard_rejects_valid_conventional_commit() {
        // Simulate what the improved fail_check guard should do:
        // "feat: description" is a valid conventional commit, so the old
        // fail_check property would wrongly assert lint(result.is_some()).
        // The fixed guard must discard this input.
        let input = "feat: description";
        let message = CommitMessage::from(format!("{input}\n# comment"));
        let result = lint(&message);
        // This IS a valid conventional commit, so lint should return None
        assert!(
            result.is_none(),
            "valid conventional commit should not be flagged, but got: {result:?}"
        );

        // Now verify the guard logic would correctly identify this as discard-worthy
        let colon_pos = input.chars().position(|x| x == ':');
        let has_non_alpha_type =
            colon_pos.is_some_and(|x| input.chars().take(x).any(|x| !x.is_ascii_alphanumeric()));
        // Old guard: only discards if non-alpha chars before colon.
        // "feat" is all alpha, so old guard does NOT discard → bug!
        assert!(
            !has_non_alpha_type,
            "old guard incorrectly keeps valid conventional commit '{input}'"
        );

        // New guard: also discard if the string parses as a valid conventional commit
        let is_valid_conventional = parse_conventional_commit(input).is_some();
        assert!(
            is_valid_conventional,
            "new guard should detect this as a valid conventional commit and discard it"
        );
    }

    #[test]
    fn test_fail_check_guard_rejects_valid_conventional_commit_with_scope() {
        let input = "feat(ui): add button";
        let message = CommitMessage::from(format!("{input}\n# comment"));
        let result = lint(&message);
        assert!(
            result.is_none(),
            "valid conventional commit with scope should pass"
        );

        let is_valid_conventional = parse_conventional_commit(input).is_some();
        assert!(
            is_valid_conventional,
            "new guard should detect this as a valid conventional commit and discard it"
        );
    }

    #[test]
    fn test_fail_check_guard_rejects_valid_conventional_commit_with_breaking() {
        let input = "refactor!: drop support";
        let message = CommitMessage::from(format!("{input}\n# comment"));
        let result = lint(&message);
        assert!(result.is_none(), "valid breaking change commit should pass");

        let is_valid_conventional = parse_conventional_commit(input).is_some();
        assert!(
            is_valid_conventional,
            "new guard should detect this as a valid conventional commit and discard it"
        );
    }

    #[test]
    fn test_fail_check_keeps_non_conventional_commits() {
        // These should NOT be discarded by the guard — they are genuinely non-conventional
        let inputs = [
            "Just a regular commit",
            "fix something",
            "no colon here",
            "fix(example: missing close paren: desc",
        ];
        for input in &inputs {
            let message = CommitMessage::from(format!("{input}\n# comment"));
            let result = lint(&message);
            assert!(
                result.is_some(),
                "non-conventional commit '{input}' should be flagged"
            );
        }
    }
}
