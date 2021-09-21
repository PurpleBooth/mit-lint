use std::convert::TryInto;

use mit_commit::CommitMessage;
use thiserror::Error;

use crate::{
    checks,
    model,
    model::{Lints, Problem},
};

/// The lints that are supported
#[derive(Debug, Eq, PartialEq, Copy, Clone, Hash, Ord, PartialOrd)]
pub enum Lint {
    /// Check for duplicated trailers
    ///
    /// # Examples
    ///
    /// Passing
    ///
    /// ```rust
    /// use indoc::indoc;
    /// use mit_commit::CommitMessage;
    /// use mit_lint::Lint;
    ///
    /// let message: &str = indoc!(
    ///     "
    ///     An example commit
    ///
    ///     This is an example commit without any duplicate trailers
    ///     "
    /// )
    /// .into();
    /// let actual = Lint::DuplicatedTrailers.lint(&CommitMessage::from(message));
    /// assert!(actual.is_none(), "Expected None, found {:?}", actual);
    /// ```
    ///
    /// Erring
    ///
    /// ```rust
    /// use indoc::indoc;
    /// use mit_commit::CommitMessage;
    /// use mit_lint::{Code, Lint, Problem};
    ///
    /// let message: &str = indoc!(
    ///     "
    ///     An example commit
    ///
    ///     This is an example commit without any duplicate trailers
    ///
    ///     Signed-off-by: Billie Thompson <email@example.com>
    ///     Signed-off-by: Billie Thompson <email@example.com>
    ///     Co-authored-by: Billie Thompson <email@example.com>
    ///     Co-authored-by: Billie Thompson <email@example.com>
    ///     "
    /// )
    /// .into();
    /// let expected = Some(Problem::new(
    ///     "Your commit message has duplicated trailers".into(),
    ///     "These are normally added accidentally when you\'re rebasing or amending to a \
    ///      commit, sometimes in the text editor, but often by git hooks.\n\nYou can fix \
    ///      this by deleting the duplicated \"Co-authored-by\", \"Signed-off-by\" fields"
    ///         .into(),
    ///     Code::DuplicatedTrailers,
    /// ));
    /// let actual = Lint::DuplicatedTrailers.lint(&CommitMessage::from(message));
    /// assert_eq!(
    ///     actual, expected,
    ///     "Expected {:?}, found {:?}",
    ///     expected, actual
    /// );
    /// ```
    DuplicatedTrailers,
    /// Check for a missing pivotal tracker id
    ///
    /// # Examples
    ///
    /// Passing
    ///
    /// ```rust
    /// use indoc::indoc;
    /// use mit_commit::CommitMessage;
    /// use mit_lint::Lint;
    ///
    /// let message: &str = indoc!(
    ///     "
    ///     An example commit [fixes #12345678]
    ///     "
    /// )
    /// .into();
    /// let actual = Lint::PivotalTrackerIdMissing.lint(&CommitMessage::from(message));
    /// assert!(actual.is_none(), "Expected None, found {:?}", actual);
    /// ```
    ///
    /// Erring
    ///
    /// ```rust
    /// use indoc::indoc;
    /// use mit_commit::CommitMessage;
    /// use mit_lint::{Code, Lint, Problem};
    ///
    /// let message: &str = indoc!(
    ///     "
    ///     An example commit
    ///
    ///     This is an example commit
    ///     "
    /// )
    /// .into();
    /// let expected = Some(Problem::new(
    ///     "Your commit message is missing a Pivotal Tracker Id".into(),
    ///     "It's important to add the ID because it allows code to be linked back to the stories it was done for, it can provide a chain of custody for code for audit purposes, and it can give future explorers of the codebase insight into the wider organisational need behind the change. We may also use it for automation purposes, like generating changelogs or notification emails.\n\nYou can fix this by adding the Id in one of the styles below to the commit message\n[Delivers #12345678]\n[fixes #12345678]\n[finishes #12345678]\n[#12345884 #12345678]\n[#12345884,#12345678]\n[#12345678],[#12345884]\nThis will address [#12345884]"
    ///         .into(),
    ///     Code::PivotalTrackerIdMissing,
    /// ));
    /// let actual = Lint::PivotalTrackerIdMissing.lint(&CommitMessage::from(message));
    /// assert_eq!(
    ///     actual, expected,
    ///     "Expected {:?}, found {:?}",
    ///     expected, actual
    /// );
    /// ```
    PivotalTrackerIdMissing,
    /// Check for a missing pivotal tracker id
    ///
    /// # Examples
    ///
    /// Passing
    ///
    /// ```rust
    /// use indoc::indoc;
    /// use mit_commit::CommitMessage;
    /// use mit_lint::Lint;
    ///
    /// let message: &str = indoc!(
    ///     "
    ///     An example commit
    ///
    ///     Relates-to: JRA-123
    ///     "
    /// )
    /// .into();
    /// let actual = Lint::JiraIssueKeyMissing.lint(&CommitMessage::from(message));
    /// assert!(actual.is_none(), "Expected None, found {:?}", actual);
    /// ```
    ///
    /// Erring
    ///
    /// ```rust
    /// use indoc::indoc;
    /// use mit_commit::CommitMessage;
    /// use mit_lint::{Code, Lint, Problem};
    ///
    /// let message: &str = indoc!(
    ///     "
    ///     An example commit
    ///
    ///     This is an example commit
    ///     "
    /// )
    /// .into();
    /// let expected = Some(Problem::new(
    ///     "Your commit message is missing a JIRA Issue Key".into(),
    ///     "It's important to add the issue key because it allows us to link code back to the motivations for doing it, and in some cases provide an audit trail for compliance purposes.\n\nYou can fix this by adding a key like `JRA-123` to the commit message"
    ///         .into(),
    ///     Code::JiraIssueKeyMissing,
    /// ));
    /// let actual = Lint::JiraIssueKeyMissing.lint(&CommitMessage::from(message));
    /// assert_eq!(
    ///     actual, expected,
    ///     "Expected {:?}, found {:?}",
    ///     expected, actual
    /// );
    /// ```
    JiraIssueKeyMissing,
    /// Check for a missing pivotal tracker id
    ///
    /// # Examples
    ///
    /// Passing
    ///
    /// ```rust
    /// use indoc::indoc;
    /// use mit_commit::CommitMessage;
    /// use mit_lint::Lint;
    ///
    /// let message: &str = indoc!(
    ///     "
    ///     An example commit
    ///
    ///     Relates-to: AnOrganisation/git-mit#642
    ///     "
    /// )
    /// .into();
    /// let actual = Lint::GitHubIdMissing.lint(&CommitMessage::from(message));
    /// assert!(actual.is_none(), "Expected None, found {:?}", actual);
    /// ```
    ///
    /// Erring
    ///
    /// ```rust
    /// use indoc::indoc;
    /// use mit_commit::CommitMessage;
    /// use mit_lint::{Code, Lint, Problem};
    ///
    /// let message: &str = indoc!(
    ///     "
    ///     An example commit
    ///
    ///     This is an example commit
    ///     "
    /// )
    /// .into();
    /// let expected = Some(Problem::new(
    ///      "Your commit message is missing a GitHub ID".into(),
    ///     "It's important to add the issue ID because it allows us to link code back to the motivations for doing it, and because we can help people exploring the repository link their issues to specific bits of code.\n\nYou can fix this by adding a ID like the following examples:\n\n#642\nGH-642\nAnUser/git-mit#642\nAnOrganisation/git-mit#642\nfixes #642\n\nBe careful just putting '#642' on a line by itself, as '#' is the default comment character"
    ///         .into(),
    ///     Code::GitHubIdMissing,
    /// ));
    /// let actual = Lint::GitHubIdMissing.lint(&CommitMessage::from(message));
    /// assert_eq!(
    ///     actual, expected,
    ///     "Expected {:?}, found {:?}",
    ///     expected, actual
    /// );
    /// ```
    GitHubIdMissing,
    /// Check for a missing pivotal tracker id
    ///
    /// # Examples
    ///
    /// Passing
    ///
    /// ```rust
    /// use indoc::indoc;
    /// use mit_commit::CommitMessage;
    /// use mit_lint::Lint;
    ///
    /// let message: &str = indoc!(
    ///     "
    ///     An example commit
    ///
    ///     Some Body Content
    ///     "
    /// )
    /// .into();
    /// let actual = Lint::SubjectNotSeparateFromBody.lint(&CommitMessage::from(message));
    /// assert!(actual.is_none(), "Expected None, found {:?}", actual);
    /// ```
    ///
    /// Erring
    ///
    /// ```rust
    /// use indoc::indoc;
    /// use mit_commit::CommitMessage;
    /// use mit_lint::{Code, Lint, Problem};
    ///
    /// let message: &str = indoc!(
    ///     "
    ///     An example commit
    ///     This is an example commit
    ///     "
    /// )
    /// .into();
    /// let expected = Some(Problem::new(
    ///       "Your commit message is missing a blank line between the subject and the body".into(),
    ///     "Most tools that render and parse commit messages, expect commit messages to be in the form of subject and body. This includes git itself in tools like git-format-patch. If you don't include this you may see strange behaviour from git and any related tools.\n\nTo fix this separate subject from body with a blank line"
    ///         .into(),
    ///     Code::SubjectNotSeparateFromBody,
    /// ));
    /// let actual = Lint::SubjectNotSeparateFromBody.lint(&CommitMessage::from(message));
    /// assert_eq!(
    ///     actual, expected,
    ///     "Expected {:?}, found {:?}",
    ///     expected, actual
    /// );
    /// ```
    SubjectNotSeparateFromBody,
    SubjectLongerThan72Characters,
    SubjectNotCapitalized,
    SubjectEndsWithPeriod,
    BodyWiderThan72Characters,
    NotConventionalCommit,
    NotEmojiLog,
}

/// The prefix we put in front of the lint when serialising
pub const CONFIG_KEY_PREFIX: &str = "mit.lint";

impl std::convert::TryFrom<&str> for Lint {
    type Error = Error;

    fn try_from(from: &str) -> Result<Self, Self::Error> {
        Lint::iterator()
            .zip(Lint::iterator().map(|lint| format!("{}", lint)))
            .filter_map(|(lint, name): (Lint, String)| if name == from { Some(lint) } else { None })
            .collect::<Vec<Lint>>()
            .first()
            .copied()
            .ok_or_else(|| Error::LintNotFound(from.into()))
    }
}

impl std::convert::From<Lint> for String {
    fn from(from: Lint) -> Self {
        format!("{}", from)
    }
}

impl From<Lint> for &str {
    fn from(lint: Lint) -> Self {
        lint.name()
    }
}

impl Lint {
    /// Get an lint's unique name
    #[must_use]
    pub fn name(self) -> &'static str {
        match self {
            Lint::DuplicatedTrailers => checks::duplicate_trailers::CONFIG,
            Lint::PivotalTrackerIdMissing => checks::missing_pivotal_tracker_id::CONFIG,
            Lint::JiraIssueKeyMissing => checks::missing_jira_issue_key::CONFIG,
            Lint::GitHubIdMissing => checks::missing_github_id::CONFIG,
            Lint::SubjectNotSeparateFromBody => checks::subject_not_separate_from_body::CONFIG,
            Lint::SubjectLongerThan72Characters => {
                checks::subject_longer_than_72_characters::CONFIG
            }
            Lint::SubjectNotCapitalized => checks::subject_not_capitalized::CONFIG,
            Lint::SubjectEndsWithPeriod => checks::subject_line_ends_with_period::CONFIG,
            Lint::BodyWiderThan72Characters => checks::body_wider_than_72_characters::CONFIG,
            Lint::NotConventionalCommit => checks::not_conventional_commit::CONFIG,
            Lint::NotEmojiLog => checks::not_emoji_log::CONFIG,
        }
    }
}

lazy_static! {
    /// All the available lints
    static ref ALL_LINTS: [Lint; 11] = [
        Lint::DuplicatedTrailers,
        Lint::PivotalTrackerIdMissing,
        Lint::JiraIssueKeyMissing,
        Lint::SubjectNotSeparateFromBody,
        Lint::GitHubIdMissing,
        Lint::SubjectLongerThan72Characters,
        Lint::SubjectNotCapitalized,
        Lint::SubjectEndsWithPeriod,
        Lint::BodyWiderThan72Characters,
        Lint::NotConventionalCommit,
        Lint::NotEmojiLog,
    ];
    /// The ones that are enabled by default
    static ref DEFAULT_ENABLED_LINTS: [Lint; 4] = [
        Lint::DuplicatedTrailers,
        Lint::SubjectNotSeparateFromBody,
        Lint::SubjectLongerThan72Characters,
        Lint::BodyWiderThan72Characters,
    ];
}
impl Lint {
    /// Iterator over all the lints
    ///
    /// # Examples
    ///
    /// ``` rust
    /// use mit_lint::Lint;
    /// assert!(Lint::iterator().next().is_some())
    /// ```
    pub fn iterator() -> impl Iterator<Item = Lint> {
        ALL_LINTS.iter().copied()
    }

    /// Check if a lint is enabled by default
    ///
    /// # Examples
    ///
    /// ``` rust
    /// use mit_lint::Lint;
    /// assert!(Lint::SubjectNotSeparateFromBody.enabled_by_default());
    /// assert!(!Lint::NotConventionalCommit.enabled_by_default());
    /// ```
    #[must_use]
    pub fn enabled_by_default(self) -> bool {
        DEFAULT_ENABLED_LINTS.contains(&self)
    }

    /// Get a key suitable for a configuration document
    ///
    /// # Examples
    ///
    /// ``` rust
    /// use mit_lint::Lint;
    /// assert_eq!(Lint::SubjectNotSeparateFromBody.config_key(), "mit.lint.subject-not-separated-from-body");
    /// ```
    #[must_use]
    pub fn config_key(self) -> String {
        format!("{}.{}", CONFIG_KEY_PREFIX, self)
    }

    /// Run this lint on a commit message
    ///
    /// # Examples
    ///
    /// ```rust
    /// use mit_commit::CommitMessage;
    /// use mit_lint::Lint;
    /// let actual =
    ///     Lint::NotConventionalCommit.lint(&CommitMessage::from("An example commit message"));
    /// assert!(actual.is_some());
    /// ```
    #[must_use]
    pub fn lint(self, commit_message: &CommitMessage) -> Option<Problem> {
        match self {
            Lint::DuplicatedTrailers => checks::duplicate_trailers::lint(commit_message),
            Lint::PivotalTrackerIdMissing => {
                checks::missing_pivotal_tracker_id::lint(commit_message)
            }
            Lint::JiraIssueKeyMissing => checks::missing_jira_issue_key::lint(commit_message),
            Lint::GitHubIdMissing => checks::missing_github_id::lint(commit_message),
            Lint::SubjectNotSeparateFromBody => {
                checks::subject_not_separate_from_body::lint(commit_message)
            }
            Lint::SubjectLongerThan72Characters => {
                checks::subject_longer_than_72_characters::lint(commit_message)
            }
            Lint::SubjectNotCapitalized => checks::subject_not_capitalized::lint(commit_message),
            Lint::SubjectEndsWithPeriod => {
                checks::subject_line_ends_with_period::lint(commit_message)
            }
            Lint::BodyWiderThan72Characters => {
                checks::body_wider_than_72_characters::lint(commit_message)
            }
            Lint::NotConventionalCommit => checks::not_conventional_commit::lint(commit_message),
            Lint::NotEmojiLog => checks::not_emoji_log::lint(commit_message),
        }
    }

    /// Try and convert a list of names into lints
    ///
    /// # Examples
    ///
    /// ```rust
    /// use mit_lint::Lint;
    /// let actual = Lint::from_names(vec!["not-emoji-log", "body-wider-than-72-characters"]);
    /// assert_eq!(
    ///     actual.unwrap(),
    ///     vec![Lint::BodyWiderThan72Characters, Lint::NotEmojiLog]
    /// );
    /// ```
    ///
    /// # Errors
    /// If the lint does not exist
    pub fn from_names(names: Vec<&str>) -> Result<Vec<Lint>, model::lints::Error> {
        let lints: Lints = names.try_into()?;
        Ok(lints.into_iter().collect())
    }
}

#[cfg(test)]
mod tests_lints {
    use std::convert::TryInto;

    use crate::model::Lint;

    #[test]
    fn it_is_convertible_to_string() {
        let string: String = Lint::PivotalTrackerIdMissing.into();
        assert_eq!("pivotal-tracker-id-missing".to_string(), string);
    }

    #[test]
    fn it_can_be_created_from_string() {
        let lint: Lint = "pivotal-tracker-id-missing".try_into().unwrap();
        assert_eq!(Lint::PivotalTrackerIdMissing, lint);
    }

    #[test]
    fn it_is_printable() {
        assert_eq!(
            "pivotal-tracker-id-missing",
            &format!("{}", Lint::PivotalTrackerIdMissing)
        );
    }

    #[test]
    fn i_can_get_all_the_lints() {
        let all: Vec<Lint> = Lint::iterator().collect();
        assert_eq!(
            all,
            vec![
                Lint::DuplicatedTrailers,
                Lint::PivotalTrackerIdMissing,
                Lint::JiraIssueKeyMissing,
                Lint::SubjectNotSeparateFromBody,
                Lint::GitHubIdMissing,
                Lint::SubjectLongerThan72Characters,
                Lint::SubjectNotCapitalized,
                Lint::SubjectEndsWithPeriod,
                Lint::BodyWiderThan72Characters,
                Lint::NotConventionalCommit,
                Lint::NotEmojiLog,
            ]
        );
    }

    #[test]
    fn i_can_get_if_a_lint_is_enabled_by_default() {
        assert!(Lint::DuplicatedTrailers.enabled_by_default());
        assert!(!Lint::PivotalTrackerIdMissing.enabled_by_default());
        assert!(!Lint::JiraIssueKeyMissing.enabled_by_default());
        assert!(Lint::SubjectNotSeparateFromBody.enabled_by_default());
        assert!(!Lint::GitHubIdMissing.enabled_by_default());
    }
}

impl std::fmt::Display for Lint {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.name())
    }
}

/// Errors
#[derive(Error, Debug)]
pub enum Error {
    #[error("Lint not found: {0}")]
    LintNotFound(String),
}
