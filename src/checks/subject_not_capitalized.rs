use std::option::Option::None;

use mit_commit::CommitMessage;

use crate::model::{Code, Problem};

/// Canonical lint ID
pub(crate) const CONFIG: &str = "subject-line-not-capitalized";
/// Advice on how to correct the problem
const HELP_MESSAGE: &str = "The subject line is a title, and as such should be \
                            capitalised.\n\nYou can fix this by capitalising the first character \
                            in the subject";
/// Description of the problem
const ERROR: &str = "Your commit message is missing a capital letter";

fn has_problem(commit_message: &CommitMessage) -> bool {
    commit_message
        .get_subject()
        .chars()
        .skip_while(|x| x.is_whitespace())
        .map(|x| x.to_uppercase().to_string() != x.to_string())
        .next()
        .unwrap_or(false)
}

pub(crate) fn lint(commit_message: &CommitMessage) -> Option<Problem> {
    if has_problem(commit_message) {
        Some(Problem::new(
            ERROR.into(),
            HELP_MESSAGE.into(),
            Code::SubjectNotCapitalized,
            commit_message,
            Some(vec![(
                "Not capitalised".to_string(),
                commit_message
                    .get_subject()
                    .chars()
                    .filter(|x| x.is_whitespace())
                    .count()
                    .saturating_sub(2),
                1_usize,
            )]),
            Some("https://git-scm.com/book/en/v2/Distributed-Git-Contributing-to-a-Project#_commit_guidelines".parse().unwrap()),
        ))
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    #![allow(clippy::wildcard_imports)]

    use super::*;
    use crate::model::{Code, Problem};

    #[test]
    fn capitalised() {
        run_test(
            "Subject Line
",
            &None,
        );
    }

    #[test]
    fn lower_case() {
        run_test(
            "subject line
",
            &Some(Problem::new(
                ERROR.into(),
                HELP_MESSAGE.into(),
                Code::SubjectNotCapitalized,
                &"subject line
"
                .into(),
                Some(vec![("Not capitalised".to_string(), 0_usize, 1_usize)]),
                Some("https://git-scm.com/book/en/v2/Distributed-Git-Contributing-to-a-Project#_commit_guidelines".parse().unwrap()),
            )),
        );
    }

    #[test]
    fn space_first() {
        run_test(
            "  subject line",
            &Some(Problem::new(
                ERROR.into(),
                HELP_MESSAGE.into(),
                Code::SubjectNotCapitalized,
                &CommitMessage::from("  subject line"),
                Some(vec![("Not capitalised".to_string(), 1_usize, 1_usize)]),
                Some("https://git-scm.com/book/en/v2/Distributed-Git-Contributing-to-a-Project#_commit_guidelines".to_string()),
            )),
        );
    }

    #[test]
    fn numbers_are_fine() {
        run_test(
            "1234567
", &None,
        );
    }

    use std::option::Option::None;

    use miette::{GraphicalReportHandler, GraphicalTheme, Report};

    #[test]
    fn formatting() {
        let message = "  an example commit\n\nexample";
        let problem = lint(&CommitMessage::from(message.to_string()));
        let actual = fmt_report(&Report::new(problem.unwrap()));
        let expected = "SubjectNotCapitalized (https://git-scm.com/book/en/v2/Distributed-Git-Contributing-to-a-Project#_commit_guidelines)

  x Your commit message is missing a capital letter
   ,-[1:1]
 1 | an example commit
   : |
   : `-- Not capitalised
 2 | 
   `----
  help: The subject line is a title, and as such should be capitalised.
        
        You can fix this by capitalising the first character in the subject
"
        .to_string();
        assert_eq!(
            actual, expected,
            "Message {:?} should have returned {:?}, found {:?}",
            message, expected, actual
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

    fn run_test(message: &str, expected: &Option<Problem>) {
        let actual = &lint(&CommitMessage::from(message));
        assert_eq!(
            actual, expected,
            "Message {:?} should have returned {:?}, found {:?}",
            message, expected, actual
        );
    }
}
