use crate::model::code::Code;

/// Information about the breaking of the lint
#[derive(Debug, Eq, PartialEq, Clone)]
pub struct Problem {
    error: String,
    tip: String,
    code: Code,
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
    /// );
    ///
    /// assert_eq!(problem.error(), "Error title".to_string())
    /// ```
    #[must_use]
    pub fn new(error: String, tip: String, code: Code) -> Problem {
        Problem { error, tip, code }
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
    /// );
    ///
    /// assert_eq!(problem.code(), &Code::BodyWiderThan72Characters)
    /// ```
    #[must_use]
    pub fn code(&self) -> &Code {
        &self.code
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
    use crate::model::{code::Code, Problem};

    #[test]
    fn test_has_error() {
        let problem = Problem::new("Some error".into(), "".into(), Code::NotConventionalCommit);
        assert_eq!(problem.error(), "Some error");
    }

    #[test]
    fn test_has_has_tip() {
        let problem = Problem::new("".into(), "Some tip".into(), Code::NotConventionalCommit);
        assert_eq!(problem.tip(), "Some tip");
    }

    #[test]
    fn test_has_has_code() {
        let problem = Problem::new("".into(), "".into(), Code::NotConventionalCommit);
        assert_eq!(problem.code(), &Code::NotConventionalCommit);
    }
}
