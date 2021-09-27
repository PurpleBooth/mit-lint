use miette::Diagnostic;
use mit_commit::CommitMessage;
use thiserror::Error;

use crate::model::code::Code;

/// Information about the breaking of the lint
#[derive(Error, Debug, Eq, PartialEq, Clone, Diagnostic)]
#[error("{error}")]
#[diagnostic(url(docsrs), code("LINT{code}"), help("{tip}"))]
pub struct Problem {
    error: String,
    tip: String,
    code: Code,
    #[source_code]
    commit_message: String,
}

impl Problem {
    /// Create a new problem
    ///
    /// # Examples
    ///
    /// ```rust
    /// use mit_lint::{Code, Problem};
    /// let problem = Problem::new(
    ///     "Error title".to_string(),
    ///     "Some advice on how to fix it".to_string(),
    ///     Code::BodyWiderThan72Characters,
    ///     &"Commit Message".into(),
    /// );
    ///
    /// assert_eq!(problem.error(), "Error title".to_string())
    /// ```
    #[must_use]
    pub fn new(error: String, tip: String, code: Code, commit_message: &CommitMessage) -> Problem {
        Problem {
            error,
            tip,
            code,
            commit_message: String::from(commit_message.clone()),
        }
    }

    /// Get the code for this problem
    ///
    /// # Examples
    ///
    /// ```rust
    /// use mit_lint::{Code, Problem};
    /// let problem = Problem::new(
    ///     "Error title".to_string(),
    ///     "Some advice on how to fix it".to_string(),
    ///     Code::BodyWiderThan72Characters,
    ///     &"Commit Message".into(),
    /// );
    ///
    /// assert_eq!(problem.code(), &Code::BodyWiderThan72Characters)
    /// ```
    #[must_use]
    pub fn code(&self) -> &Code {
        &self.code
    }

    /// Get the commit message for this problem
    ///
    /// # Examples
    ///
    /// ```rust
    /// use mit_commit::CommitMessage;
    /// use mit_lint::{Code, Problem};
    /// let problem = Problem::new(
    ///     "Error title".to_string(),
    ///     "Some advice on how to fix it".to_string(),
    ///     Code::BodyWiderThan72Characters,
    ///     &"Commit Message".into(),
    /// );
    ///
    /// assert_eq!(
    ///     problem.commit_message(),
    ///     CommitMessage::from("Commit Message")
    /// )
    /// ```
    #[must_use]
    pub fn commit_message(&self) -> CommitMessage {
        self.commit_message.clone().into()
    }

    /// Get the descriptive title for this error
    ///
    /// # Examples
    ///
    /// ```rust
    /// use mit_lint::{Code, Problem};
    /// let problem = Problem::new(
    ///     "Error title".to_string(),
    ///     "Some advice on how to fix it".to_string(),
    ///     Code::BodyWiderThan72Characters,
    ///     &"Commit Message".into(),
    /// );
    ///
    /// assert_eq!(problem.error(), "Error title".to_string())
    /// ```
    #[must_use]
    pub fn error(&self) -> &str {
        &self.error
    }

    /// Get advice on how to fix the problem
    ///
    /// This should be a description of why this is a problem, and how to fix it
    ///
    /// # Examples
    ///
    /// ```rust
    /// use mit_lint::{Code, Problem};
    /// let problem = Problem::new(
    ///     "Error title".to_string(),
    ///     "Some advice on how to fix it".to_string(),
    ///     Code::BodyWiderThan72Characters,
    ///     &"Commit Message".into(),
    /// );
    ///
    /// assert_eq!(problem.tip(), "Some advice on how to fix it".to_string())
    /// ```
    #[must_use]
    pub fn tip(&self) -> &str {
        &self.tip
    }
}

#[cfg(test)]
mod tests {
    use mit_commit::CommitMessage;

    use crate::model::{code::Code, Problem};

    #[test]
    fn test_has_error() {
        let problem = Problem::new(
            "Some error".into(),
            "".into(),
            Code::NotConventionalCommit,
            &"".into(),
        );
        assert_eq!(problem.error(), "Some error");
    }

    #[test]
    fn test_has_has_tip() {
        let problem = Problem::new(
            "".into(),
            "Some tip".into(),
            Code::NotConventionalCommit,
            &"".into(),
        );
        assert_eq!(problem.tip(), "Some tip");
    }

    #[test]
    fn test_has_has_code() {
        let problem = Problem::new(
            "".into(),
            "".into(),
            Code::NotConventionalCommit,
            &"".into(),
        );
        assert_eq!(problem.code(), &Code::NotConventionalCommit);
    }

    #[test]
    fn test_it_contains_the_triggering_message() {
        let problem = Problem::new(
            "".into(),
            "".into(),
            Code::NotConventionalCommit,
            &CommitMessage::from("Commit message"),
        );
        assert_eq!(
            problem.commit_message(),
            CommitMessage::from("Commit message")
        );
    }
}
