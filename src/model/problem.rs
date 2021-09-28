use std::{borrow::Borrow, fmt::Display};

use miette::{Diagnostic, LabeledSpan, SourceCode};
use mit_commit::CommitMessage;
use thiserror::Error;

use crate::model::code::Code;

/// Information about the breaking of the lint
#[derive(Error, Debug, Eq, PartialEq, Clone)]
#[error("{error}")]
pub struct Problem {
    error: String,
    tip: String,
    code: Code,
    commit_message: String,
    labels: Option<Vec<(String, usize, usize)>>,
    url: Option<String>,
}

impl Diagnostic for Problem {
    /// Unique diagnostic code that can be used to look up more information
    /// about this Diagnostic. Ideally also globally unique, and documented in
    /// the toplevel crate's documentation for easy searching. Rust path
    /// format (`foo::bar::baz`) is recommended, but more classic codes like
    /// `E0123` or Enums will work just fine.
    fn code<'a>(&'a self) -> Option<Box<dyn Display + 'a>> {
        Some(Box::new(format!("{:?}", self.code)))
    }

    /// Additional help text related to this Diagnostic. Do you have any
    /// advice for the poor soul who's just run into this issue?
    fn help<'a>(&'a self) -> Option<Box<dyn Display + 'a>> {
        Some(Box::new(&self.tip))
    }

    fn source_code(&self) -> Option<&dyn SourceCode> {
        Some(&self.commit_message)
    }

    fn labels(&self) -> Option<Box<dyn Iterator<Item = LabeledSpan> + '_>> {
        match &self.labels {
            None => None,
            Some(labels) => {
                Some(Box::new(labels.iter().map(|(label, offset, len)| {
                    LabeledSpan::new(Some(label.clone()), *offset, *len)
                }))
                    as Box<dyn Iterator<Item = LabeledSpan> + '_>)
            }
        }
    }

    fn url<'a>(&'a self) -> Option<Box<dyn Display + 'a>> {
        match self.url.borrow() {
            None => None,
            Some(url) => Some(Box::new(url)),
        }
    }
}

impl Problem {
    /// Create a new problem
    ///
    /// # Examples
    ///
    /// ```rust
    /// use std::option::Option::None;
    ///
    /// use mit_lint::{Code, Problem};
    /// let problem = Problem::new(
    ///     "Error title".to_string(),
    ///     "Some advice on how to fix it".to_string(),
    ///     Code::BodyWiderThan72Characters,
    ///     &"Commit Message".into(),
    ///     None,
    ///     None,
    /// );
    ///
    /// assert_eq!(problem.error(), "Error title".to_string())
    /// ```
    #[must_use]
    pub fn new(
        error: String,
        tip: String,
        code: Code,
        commit_message: &CommitMessage,
        labels: Option<Vec<(String, usize, usize)>>,
        url: Option<String>,
    ) -> Problem {
        Problem {
            error,
            tip,
            code,
            commit_message: String::from(commit_message.clone()),
            labels,
            url,
        }
    }

    /// Get the code for this problem
    ///
    /// # Examples
    ///
    /// ```rust
    /// use std::option::Option::None;
    ///
    /// use mit_lint::{Code, Problem};
    /// let problem = Problem::new(
    ///     "Error title".to_string(),
    ///     "Some advice on how to fix it".to_string(),
    ///     Code::BodyWiderThan72Characters,
    ///     &"Commit Message".into(),
    ///     None,
    ///     None,
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
    /// use std::option::Option::None;
    ///
    /// use mit_commit::CommitMessage;
    /// use mit_lint::{Code, Problem};
    /// let problem = Problem::new(
    ///     "Error title".to_string(),
    ///     "Some advice on how to fix it".to_string(),
    ///     Code::BodyWiderThan72Characters,
    ///     &"Commit Message".into(),
    ///     None,
    ///     None,
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
    /// use std::option::Option::None;
    ///
    /// use mit_lint::{Code, Problem};
    /// let problem = Problem::new(
    ///     "Error title".to_string(),
    ///     "Some advice on how to fix it".to_string(),
    ///     Code::BodyWiderThan72Characters,
    ///     &"Commit Message".into(),
    ///     None,
    ///     None,
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
    /// use std::option::Option::None;
    ///
    /// use mit_lint::{Code, Problem};
    /// let problem = Problem::new(
    ///     "Error title".to_string(),
    ///     "Some advice on how to fix it".to_string(),
    ///     Code::BodyWiderThan72Characters,
    ///     &"Commit Message".into(),
    ///     None,
    ///     None,
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
    use std::option::Option::None;

    use miette::Diagnostic;
    use mit_commit::CommitMessage;

    use crate::model::{code::Code, Problem};

    #[test]
    fn test_has_error() {
        let problem = Problem::new(
            "Some error".into(),
            "".into(),
            Code::NotConventionalCommit,
            &"".into(),
            None,
            None,
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
            None,
            None,
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
            None,
            None,
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
            None,
            None,
        );
        assert_eq!(
            problem.commit_message(),
            CommitMessage::from("Commit message")
        );
    }
    #[test]
    fn test_it_contains_the_labels() {
        let problem = Problem::new(
            "".into(),
            "".into(),
            Code::NotConventionalCommit,
            &CommitMessage::from("Commit message"),
            Some(vec![("String".to_string(), 10_usize, 20_usize)]),
            None,
        );
        assert_eq!(
            problem
                .labels()
                .unwrap()
                .map(|x| (x.label().unwrap().to_string(), x.offset(), x.len()))
                .collect::<Vec<_>>(),
            vec![("String".to_string(), 10_usize, 20_usize)]
        );
    }
}
