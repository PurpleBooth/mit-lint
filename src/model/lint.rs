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
    /// use mit_commit::CommitMessage;
    /// use mit_lint::Lint;
    ///
    /// let message: &str = "An example commit
    ///
    /// This is an example commit without any duplicate trailers
    /// "
    /// .into();
    /// let actual = Lint::DuplicatedTrailers.lint(&CommitMessage::from(message));
    /// assert!(actual.is_none(), "Expected None, found {:?}", actual);
    /// ```
    ///
    /// Erring
    ///
    /// ```rust
    /// use mit_commit::CommitMessage;
    /// use mit_lint::{Code, Lint, Problem};
    ///
    /// let message: &str = "An example commit
    ///
    /// This is an example commit without any duplicate trailers
    ///
    /// Signed-off-by: Billie Thompson <email@example.com>
    /// Signed-off-by: Billie Thompson <email@example.com>
    /// Co-authored-by: Billie Thompson <email@example.com>
    /// Co-authored-by: Billie Thompson <email@example.com>
    /// "
    /// .into();
    /// let expected = Some(Problem::new(
    ///     "Your commit message has duplicated trailers".into(),
    ///     "These are normally added accidentally when you\'re rebasing or amending to a \
    ///      commit, sometimes in the text editor, but often by git hooks.\n\nYou can fix \
    ///      this by deleting the duplicated \"Co-authored-by\", \"Signed-off-by\" fields"
    ///         .into(),
    ///     Code::DuplicatedTrailers,
    ///     &message.into(),
    ///     Some(vec![
    ///         ("Duplicated `Co-authored-by`".to_string(), 231, 51),
    ///         ("Duplicated `Signed-off-by`".to_string(), 128, 50),
    ///     ]),
    ///     None,
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
    /// use mit_commit::CommitMessage;
    /// use mit_lint::Lint;
    ///
    /// let message: &str = "An example commit [fixes #12345678]
    /// "
    /// .into();
    /// let actual = Lint::PivotalTrackerIdMissing.lint(&CommitMessage::from(message));
    /// assert!(actual.is_none(), "Expected None, found {:?}", actual);
    /// ```
    ///
    /// Erring
    ///
    /// ```rust
    /// use mit_commit::CommitMessage;
    /// use mit_lint::{Code, Lint, Problem};
    ///
    /// let message: &str = "An example commit
    ///
    /// This is an example commit
    /// "
    ///
    /// .into();
    /// let expected = Some(Problem::new(
    ///     "Your commit message is missing a Pivotal Tracker ID".into(),
    ///     "It's important to add the ID because it allows code to be linked back to the stories it was done for, it can provide a chain of custody for code for audit purposes, and it can give future explorers of the codebase insight into the wider organisational need behind the change. We may also use it for automation purposes, like generating changelogs or notification emails.\n\nYou can fix this by adding the Id in one of the styles below to the commit message\n[Delivers #12345678]\n[fixes #12345678]\n[finishes #12345678]\n[#12345884 #12345678]\n[#12345884,#12345678]\n[#12345678],[#12345884]\nThis will address [#12345884]"
    ///         .into(),
    ///     Code::PivotalTrackerIdMissing,
    ///     &message.into(),
    ///     Some(vec![("No Pivotal Tracker ID".to_string(), 19, 26)]),
    ///     None,
    /// ));
    /// let actual = Lint::PivotalTrackerIdMissing.lint(&CommitMessage::from(message));
    /// assert_eq!(
    ///     actual, expected,
    ///     "Expected {:?}, found {:?}",
    ///     expected, actual
    /// );
    /// ```
    PivotalTrackerIdMissing,
    /// Check for a missing jira issue key
    ///
    /// # Examples
    ///
    /// Passing
    ///
    /// ```rust
    /// use mit_commit::CommitMessage;
    /// use mit_lint::Lint;
    ///
    /// let message: &str = "An example commit
    ///
    /// Relates-to: JRA-123
    /// "
    /// .into();
    /// let actual = Lint::JiraIssueKeyMissing.lint(&CommitMessage::from(message));
    /// assert!(actual.is_none(), "Expected None, found {:?}", actual);
    /// ```
    ///
    /// Erring
    ///
    /// ```rust
    /// use mit_commit::CommitMessage;
    /// use mit_lint::{Code, Lint, Problem};
    ///
    /// let message: &str = "An example commit
    ///
    /// This is an example commit
    /// "
    ///
    /// .into();
    /// let expected = Some(Problem::new(
    ///     "Your commit message is missing a JIRA Issue Key".into(),
    ///     "It's important to add the issue key because it allows us to link code back to the motivations for doing it, and in some cases provide an audit trail for compliance purposes.\n\nYou can fix this by adding a key like `JRA-123` to the commit message"
    ///         .into(),
    ///     Code::JiraIssueKeyMissing,&message.into(),Some(vec![("No JIRA Issue Key".to_string(), 19, 26)]),None,
    /// ));
    /// let actual = Lint::JiraIssueKeyMissing.lint(&CommitMessage::from(message));
    /// assert_eq!(
    ///     actual, expected,
    ///     "Expected {:?}, found {:?}",
    ///     expected, actual
    /// );
    /// ```
    JiraIssueKeyMissing,
    /// Check for a missing github id
    ///
    /// # Examples
    ///
    /// Passing
    ///
    /// ```rust
    /// use mit_commit::CommitMessage;
    /// use mit_lint::Lint;
    ///
    /// let message: &str = "An example commit
    ///
    /// Relates-to: AnOrganisation/git-mit#642
    /// "
    /// .into();
    /// let actual = Lint::GitHubIdMissing.lint(&CommitMessage::from(message));
    /// assert!(actual.is_none(), "Expected None, found {:?}", actual);
    /// ```
    ///
    /// Erring
    ///
    /// ```rust
    /// use mit_commit::CommitMessage;
    /// use mit_lint::{Code, Lint, Problem};
    ///
    /// let message: &str = "An example commit
    ///
    /// This is an example commit
    /// "
    ///
    /// .into();
    /// let expected = Some(Problem::new(
    ///      "Your commit message is missing a GitHub ID".into(),
    ///     "It's important to add the issue ID because it allows us to link code back to the motivations for doing it, and because we can help people exploring the repository link their issues to specific bits of code.\n\nYou can fix this by adding a ID like the following examples:\n\n#642\nGH-642\nAnUser/git-mit#642\nAnOrganisation/git-mit#642\nfixes #642\n\nBe careful just putting '#642' on a line by itself, as '#' is the default comment character"
    ///         .into(),
    ///     Code::GitHubIdMissing,&message.into(),Some(vec![("No GitHub ID".to_string(), 19, 26)]),None,
    /// ));
    /// let actual = Lint::GitHubIdMissing.lint(&CommitMessage::from(message));
    /// assert_eq!(
    ///     actual, expected,
    ///     "Expected {:?}, found {:?}",
    ///     expected, actual
    /// );
    /// ```
    GitHubIdMissing,
    /// Subject being not being seperated from the body
    ///
    /// # Examples
    ///
    /// Passing
    ///
    /// ```rust
    /// use mit_commit::CommitMessage;
    /// use mit_lint::Lint;
    ///
    /// let message: &str = "An example commit
    ///
    /// Some Body Content
    /// "
    /// .into();
    /// let actual = Lint::SubjectNotSeparateFromBody.lint(&CommitMessage::from(message));
    /// assert!(actual.is_none(), "Expected None, found {:?}", actual);
    /// ```
    ///
    /// Erring
    ///
    /// ```rust
    /// use mit_commit::CommitMessage;
    /// use mit_lint::{Code, Lint, Problem};
    ///
    /// let message: &str = "An example commit
    /// This is an example commit
    /// "
    /// .into();
    /// let expected = Some(Problem::new(
    ///       "Your commit message is missing a blank line between the subject and the body".into(),
    ///     "Most tools that render and parse commit messages, expect commit messages to be in the form of subject and body. This includes git itself in tools like git-format-patch. If you don't include this you may see strange behaviour from git and any related tools.\n\nTo fix this separate subject from body with a blank line"
    ///         .into(),
    ///     Code::SubjectNotSeparateFromBody,&message.into(),Some(vec![("Missing blank line".to_string(), 18, 25)]),None,
    /// ));
    /// let actual = Lint::SubjectNotSeparateFromBody.lint(&CommitMessage::from(message));
    /// assert_eq!(
    ///     actual, expected,
    ///     "Expected {:?}, found {:?}",
    ///     expected, actual
    /// );
    /// ```
    SubjectNotSeparateFromBody,
    /// Check for a long subject line
    ///
    /// # Examples
    ///
    /// Passing
    ///
    /// ```rust
    /// use mit_commit::CommitMessage;
    /// use mit_lint::Lint;
    ///
    /// let message: &str = "An example commit
    ///
    /// Some Body Content
    /// "
    /// .into();
    /// let actual = Lint::SubjectLongerThan72Characters.lint(&CommitMessage::from(message));
    /// assert!(actual.is_none(), "Expected None, found {:?}", actual);
    /// ```
    ///
    /// Erring
    ///
    /// ```
    /// use mit_commit::CommitMessage;
    /// use mit_lint::{Code, Lint, Problem};
    ///
    /// let message:String = "x".repeat(73).into();
    /// let expected = Some(Problem::new(
    ///       "Your subject is longer than 72 characters".into(),
    ///     "It's important to keep the subject of the commit less than 72 characters because when you look at the git log, that's where it truncates the message. This means that people won't get the entirety of the information in your commit.\n\nPlease keep the subject line 72 characters or under"
    ///         .into(),
    ///     Code::SubjectLongerThan72Characters,&message.clone().into(),Some(vec![("Too long".to_string(), 73, 1)]),None,
    /// ));
    /// let actual = Lint::SubjectLongerThan72Characters.lint(&CommitMessage::from(message));
    /// assert_eq!(
    ///     actual, expected,
    ///     "Expected {:?}, found {:?}",
    ///     expected, actual
    /// );
    /// ```
    SubjectLongerThan72Characters,
    /// Check for a non-capitalised subject
    ///
    /// # Examples
    ///
    /// Passing
    ///
    /// ```rust
    /// use mit_commit::CommitMessage;
    /// use mit_lint::Lint;
    ///
    /// let message: &str = "An example commit\n".into();
    /// let actual = Lint::SubjectNotCapitalized.lint(&CommitMessage::from(message));
    /// assert!(actual.is_none(), "Expected None, found {:?}", actual);
    /// ```
    ///
    /// Erring
    ///
    /// ```rust
    /// use mit_commit::CommitMessage;
    /// use mit_lint::{Code, Lint, Problem};
    ///
    /// let message: &str =
    ///     "an example commit\n"
    /// .into();
    /// let expected = Some(
    ///     Problem::new(
    ///         "Your commit message is missing a capital letter".into(),
    ///         "The subject line is a title, and as such should be capitalised.\n\nYou can fix this by capitalising the first character in the subject".into(),
    ///     Code::SubjectNotCapitalized,&message.into(),Some(vec![("Not capitalised".to_string(), 0, 1)]),
    ///     None,
    /// )
    /// );
    /// let actual = Lint::SubjectNotCapitalized.lint(&CommitMessage::from(message));
    /// assert_eq!(
    ///     actual, expected,
    ///     "Expected {:?}, found {:?}",
    ///     expected, actual
    /// );
    /// ```
    SubjectNotCapitalized,
    /// Check for period at the end of the subject
    ///
    /// # Examples
    ///
    /// Passing
    ///
    /// ```rust
    /// use mit_commit::CommitMessage;
    /// use mit_lint::Lint;
    ///
    /// let message: &str = "An example commit\n".into();
    /// let actual = Lint::SubjectEndsWithPeriod.lint(&CommitMessage::from(message));
    /// assert!(actual.is_none(), "Expected None, found {:?}", actual);
    /// ```
    ///
    /// Erring
    ///
    /// ```rust
    /// use mit_commit::CommitMessage;
    /// use mit_lint::{Code, Lint, Problem};
    ///
    /// let message: &str =
    ///     "An example commit.\n".into();
    /// let expected = Some(
    /// Problem::new(
    ///     "Your commit message ends with a period".into(),
    ///     "It's important to keep your commits short, because we only have a limited number of characters to use (72) before the subject line is truncated. Full stops aren't normally in subject lines, and take up an extra character, so we shouldn't use them in commit message subjects.\n\nYou can fix this by removing the period"
    ///         .into(),
    ///     Code::SubjectEndsWithPeriod,&message.into(),
    ///     Some(vec![("Unneeded period".to_string(), 17, 1)]),
    ///     None,
    /// )
    /// );
    /// let actual = Lint::SubjectEndsWithPeriod.lint(&CommitMessage::from(message));
    /// assert_eq!(
    ///     actual, expected,
    ///     "Expected {:?}, found {:?}",
    ///     expected, actual
    /// );
    /// ```
    SubjectEndsWithPeriod,
    /// Check for a long body line
    ///
    /// # Examples
    ///
    /// Passing
    ///
    /// ```rust
    /// use mit_commit::CommitMessage;
    /// use mit_lint::Lint;
    ///
    /// let message: &str = "An example commit\n\nSome Body Content\n".into();
    /// let actual = Lint::BodyWiderThan72Characters.lint(&CommitMessage::from(message));
    /// assert!(actual.is_none(), "Expected None, found {:?}", actual);
    /// ```
    ///
    /// Erring
    ///
    /// ```rust
    /// use mit_commit::CommitMessage;
    /// use mit_lint::{Code, Lint, Problem};
    ///
    /// let message:String = ["Subject".to_string(), "x".repeat(73).into()].join("\n\n");
    /// let expected = Some(Problem::new(
    ///   "Your commit has a body wider than 72 characters".into(),
    ///     "It's important to keep the body of the commit narrower than 72 characters because when you look at the git log, that's where it truncates the message. This means that people won't get the entirety of the information in your commit.\n\nYou can fix this by making the lines in your body no more than 72 characters"
    ///         .into(),
    ///     Code::BodyWiderThan72Characters,&message.clone().into(),
    ///     Some(vec![("Too long".parse().unwrap(), 81, 1)]),
    /// None
    /// ));
    /// let actual = Lint::BodyWiderThan72Characters.lint(&CommitMessage::from(message));
    /// assert_eq!(
    ///     actual, expected,
    ///     "Expected {:?}, found {:?}",
    ///     expected, actual
    /// );
    /// ```
    BodyWiderThan72Characters,
    /// Check for commits following the conventional standard
    ///
    /// # Examples
    ///
    /// Passing
    ///
    /// ```rust
    /// use mit_commit::CommitMessage;
    /// use mit_lint::Lint;
    ///
    /// let message: &str = "refactor: An example commit\n\nSome Body Content\n".into();
    /// let actual = Lint::NotConventionalCommit.lint(&CommitMessage::from(message));
    /// assert!(actual.is_none(), "Expected None, found {:?}", actual);
    /// ```
    ///
    /// Erring
    ///
    /// ```rust
    /// use mit_commit::CommitMessage;
    /// use mit_lint::{Code, Lint, Problem};
    ///
    /// let message: &str =
    ///     "An example commit\n\nSome Body Content\n"
    /// .into();
    /// let expected = Some(Problem::new(
    ///       "Your commit message isn't in conventional style".into(),
    ///      "It's important to follow the conventional commit style when creating your commit message. By using this style we can automatically calculate the version of software using deployment pipelines, and also generate changelogs and other useful information without human interaction.\n\nYou can fix it by following style\n\n<type>[optional scope]: <description>\n\n[optional body]\n\n[optional footer(s)]"
    ///         .into(),
    ///     Code::NotConventionalCommit,&message.into(),Some(vec![("Not conventional".to_string(), 0, 17)]),Some("https://www.conventionalcommits.org/".to_string()),
    /// ));
    /// let actual = Lint::NotConventionalCommit.lint(&CommitMessage::from(message));
    /// assert_eq!(
    ///     actual, expected,
    ///     "Expected {:?}, found {:?}",
    ///     expected, actual
    /// );
    /// ```
    NotConventionalCommit,
    /// Check for commits following the emoji log standard
    ///
    /// # Examples
    ///
    /// Passing
    ///
    /// ```rust
    /// use mit_commit::CommitMessage;
    /// use mit_lint::Lint;
    ///
    /// let message: &str = "📖 DOC: An example commit\n\nSome Body Content\n".into();
    /// let actual = Lint::NotEmojiLog.lint(&CommitMessage::from(message));
    /// assert!(actual.is_none(), "Expected None, found {:?}", actual);
    /// ```
    ///
    /// Erring
    ///
    /// ```rust
    /// use mit_commit::CommitMessage;
    /// use mit_lint::{Code, Lint, Problem};
    ///
    /// let message: &str =
    ///     "An example commit\n\nSome Body Content\n"
    /// .into();
    /// let expected = Some(
    /// Problem::new(
    ///        "Your commit message isn't in emoji log style".into(),
    ///      "It's important to follow the emoji log style when creating your commit message. By using this style we can automatically generate changelogs.\n\nYou can fix it using one of the prefixes:\n\n\n📦 NEW:\n👌 IMPROVE:\n🐛 FIX:\n📖 DOC:\n🚀 RELEASE:\n🤖 TEST:\n‼\u{fe0f} BREAKING:"
    ///         .into(),
    ///     Code::NotEmojiLog,&message.into(),Some(vec![("Not emoji log".to_string(), 0, 17)]),Some("https://github.com/ahmadawais/Emoji-Log".to_string()),
    /// ));
    /// let actual = Lint::NotEmojiLog.lint(&CommitMessage::from(message));
    /// assert_eq!(
    ///     actual, expected,
    ///     "Expected {:?}, found {:?}",
    ///     expected, actual
    /// );
    /// ```
    NotEmojiLog,
}

/// The prefix we put in front of the lint when serialising
pub const CONFIG_KEY_PREFIX: &str = "mit.lint";

impl std::convert::TryFrom<&str> for Lint {
    type Error = Error;

    fn try_from(from: &str) -> Result<Self, Self::Error> {
        Lint::all_lints()
            .zip(Lint::all_lints().map(|lint| format!("{}", lint)))
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
    /// ```rust
    /// use mit_lint::Lint;
    /// assert!(Lint::all_lints().next().is_some())
    /// ```
    pub fn all_lints() -> impl Iterator<Item = Lint> {
        ALL_LINTS.iter().copied()
    }

    /// Iterator over all the lints
    ///
    /// # Examples
    ///
    /// ```rust
    /// use mit_lint::Lint;
    /// assert!(Lint::iterator().next().is_some())
    /// ```
    #[deprecated(since = "0.1.5", note = "iterator was an unusual name. Use all_lints")]
    pub fn iterator() -> impl Iterator<Item = Lint> {
        Lint::all_lints()
    }

    /// Check if a lint is enabled by default
    ///
    /// # Examples
    ///
    /// ```rust
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
    /// ```rust
    /// use mit_lint::Lint;
    /// assert_eq!(
    ///     Lint::SubjectNotSeparateFromBody.config_key(),
    ///     "mit.lint.subject-not-separated-from-body"
    /// );
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
        let all: Vec<Lint> = Lint::all_lints().collect();
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
