use std::option::Option::None;

use mit_commit::CommitMessage;

use crate::model::{Code, Problem};

/// Canonical lint ID
pub(crate) const CONFIG: &str = "subject-line-ends-with-period";
/// Description of the problem
const ERROR: &str = "Your commit message ends with a period";
/// Advice on how to correct the problem
const HELP_MESSAGE: &str = "It's important to keep your commits short, because we only have a \
                            limited number of characters to use (72) before the subject line is \
                            truncated. Full stops aren't normally in subject lines, and take up \
                            an extra character, so we shouldn't use them in commit message \
                            subjects.\n\nYou can fix this by removing the period";

fn has_problem(commit_message: &CommitMessage) -> bool {
    matches!(
        commit_message
            .get_subject()
            .chars()
            .rev()
            .find(|ch| !ch.is_whitespace()),
        Some('.')
    )
}

pub(crate) fn lint(commit_message: &CommitMessage) -> Option<Problem> {
    if has_problem(commit_message) {
        let subject = commit_message.get_subject().to_string();
        Some(Problem::new(
            ERROR.into(),
            HELP_MESSAGE.into(),
            Code::SubjectEndsWithPeriod,
            commit_message,
            Some(vec![(
                "Unneeded period".to_string(),
                subject.len()
                    - subject
                        .chars()
                        .rev()
                        .filter(|ch| ch == &'.' || ch.is_whitespace())
                        .count()
                        .saturating_sub(2),
                subject
                    .chars()
                    .rev()
                    .filter(|ch| !ch.is_whitespace())
                    .take_while(|ch| ch == &'.')
                    .count(),
            )]),
            None,
        ))
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    #![allow(clippy::wildcard_imports)]

    use indoc::indoc;

    use super::*;
    use crate::model::{Code, Problem};

    #[test]
    fn subject_does_not_end_with_period() {
        run_test(
            indoc!(
                "
                Subject Line
                "
            ),
            &None,
        );
    }

    #[test]
    fn subject_ends_with_period() {
        let message = indoc!(
            "
            Subject Line.
            "
        );
        run_test(
            message,
            &Some(Problem::new(
                ERROR.into(),
                HELP_MESSAGE.into(),
                Code::SubjectEndsWithPeriod,
                &message.into(),
                Some(vec![("Unneeded period".to_string(), 13_usize, 1_usize)]),
                None,
            )),
        );
    }

    #[test]
    fn subject_has_period_then_whitespace() {
        let message = "Subject Line. ";
        run_test(
            message,
            &Some(Problem::new(
                ERROR.into(),
                HELP_MESSAGE.into(),
                Code::SubjectEndsWithPeriod,
                &message.into(),
                Some(vec![("Unneeded period".to_string(), 13_usize, 1_usize)]),
                None,
            )),
        );
    }

    #[test]
    fn subject_has_multiple_periods() {
        let message = "Subject Line... ";
        run_test(
            message,
            &Some(Problem::new(
                ERROR.into(),
                HELP_MESSAGE.into(),
                Code::SubjectEndsWithPeriod,
                &message.into(),
                Some(vec![("Unneeded period".to_string(), 13_usize, 3_usize)]),
                None,
            )),
        );
    }

    use std::option::Option::None;

    use miette::{GraphicalReportHandler, GraphicalTheme, Report};

    #[test]
    fn formatting() {
        let message = indoc!(
            "
            An example commit... 

            This is an example commit
            "
        );
        let problem = lint(&CommitMessage::from(message.to_string()));
        let actual = fmt_report(&Report::new(problem.unwrap()));
        let expected = indoc!(
            "
            SubjectEndsWithPeriod
            
              \u{d7} Your commit message ends with a period
               \u{256d}\u{2500}[1:1]
             1 \u{2502} ... 
               \u{b7} \u{2500}\u{252c}\u{2500}
               \u{b7}  \u{2570}\u{2500}\u{2500} Unneeded period
             2 \u{2502} 
               \u{2570}\u{2500}\u{2500}\u{2500}\u{2500}
              help: It's important to keep your commits short, because we only have a
                    limited number of characters to use (72) before the subject line
                    is truncated. Full stops aren't normally in subject lines, and take
                    up an extra character, so we shouldn't use them in commit message
                    subjects.
                    
                    You can fix this by removing the period
            "
        )
        .to_string();
        assert_eq!(
            actual, expected,
            "Message {:?} should have returned {:?}, found {:?}",
            message, expected, actual
        );
    }

    fn fmt_report(diag: &Report) -> String {
        let mut out = String::new();
        GraphicalReportHandler::new_themed(GraphicalTheme::unicode_nocolor())
            .with_width(80)
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
