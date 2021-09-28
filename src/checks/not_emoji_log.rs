use std::option::Option::None;

use indoc::indoc;
use mit_commit::CommitMessage;

use crate::model::{Code, Problem};

/// Canonical lint ID
pub(crate) const CONFIG: &str = "not-emoji-log";

/// Advice on how to correct the problem
const HELP_MESSAGE: &str = indoc!(
    "
    It's important to follow the emoji log style when creating your commit message. By using this \
    style we can automatically generate changelogs.

    You can fix it using one of the prefixes:

    
    \u{1f4e6} NEW:
    \u{1f44c} IMPROVE:
    \u{1f41b} FIX:
    \u{1f4d6} DOC:
    \u{1f680} RELEASE:
    \u{1f916} TEST:
    \u{203c}\u{fe0f} BREAKING:"
);
/// Description of the problem
const ERROR: &str = "Your commit message isn't in emoji log style";

const PREFIXES: &[&str] = &[
    "\u{1f4e6} NEW: ",
    "\u{1f44c} IMPROVE: ",
    "\u{1f41b} FIX: ",
    "\u{1f4d6} DOC: ",
    "\u{1f680} RELEASE: ",
    "\u{1f916} TEST: ",
    "\u{203c}\u{fe0f} BREAKING: ",
];

pub(crate) fn lint(commit_message: &CommitMessage) -> Option<Problem> {
    if PREFIXES
        .iter()
        .any(|x| commit_message.get_subject().to_string().starts_with(x))
    {
        None
    } else {
        let commit_text = String::from(commit_message.clone());
        Some(Problem::new(
            ERROR.into(),
            HELP_MESSAGE.into(),
            Code::NotEmojiLog,
            commit_message,
            Some(vec![(
                "Not emoji log".to_string(),
                0_usize,
                commit_text.lines().next().map(str::len).unwrap(),
            )]),
            Some("https://github.com/ahmadawais/Emoji-Log".to_string()),
        ))
    }
}

#[cfg(test)]
mod test {

    use indoc::indoc;

    use super::*;
    use crate::model::{Code, Problem};

    #[test]
    fn new() {
        run_lint(
            indoc!(
                "
                \u{1f4e6} NEW: An example commit

                This is an example commit
                "
            ),
            &None,
        );
    }

    #[test]
    fn improve() {
        run_lint(
            indoc!(
                "
                \u{1f44c} IMPROVE: An example commit

                This is an example commit
                "
            ),
            &None,
        );
    }

    #[test]
    fn fix() {
        run_lint(
            indoc!(
                "
                \u{1f41b} FIX: An example commit

                This is an example commit
                "
            ),
            &None,
        );
    }

    #[test]
    fn docs() {
        run_lint(
            indoc!(
                "
                \u{1f4d6} DOC: An example commit

                This is an example commit
                "
            ),
            &None,
        );
    }

    #[test]
    fn release() {
        run_lint(
            indoc!(
                "
                \u{1f680} RELEASE: An example commit

                This is an example commit
                "
            ),
            &None,
        );
    }

    #[test]
    fn test() {
        run_lint(
            indoc!(
                "
                \u{1f916} TEST: An example commit

                This is an example commit
                "
            ),
            &None,
        );
    }

    #[test]
    fn bc() {
        run_lint(
            indoc!(
                "
                \u{203c}\u{fe0f} BREAKING: An example commit

                This is an example commit
                "
            ),
            &None,
        );
    }

    #[test]
    fn no_gap() {
        let message = indoc!(
            "
            \u{203c}\u{fe0f} BREAKING:An example commit

            This is an example commit
            "
        );
        run_lint(
            message,
            &Some(Problem::new(
                ERROR.into(),
                HELP_MESSAGE.into(),
                Code::NotEmojiLog,
                &message.into(),
                Some(vec![("Not emoji log".to_string(), 0_usize, 33_usize)]),
                Some("https://github.com/ahmadawais/Emoji-Log".to_string()),
            )),
        );
    }

    #[test]
    fn unknown_emoji() {
        let message = indoc!(
            "
            \u{1f408} UNKNOWN: An example commit

            This is an example commit
            "
        );
        run_lint(
            message,
            &Some(Problem::new(
                ERROR.into(),
                HELP_MESSAGE.into(),
                Code::NotEmojiLog,
                &message.into(),
                Some(vec![("Not emoji log".to_string(), 0_usize, 31_usize)]),
                Some("https://github.com/ahmadawais/Emoji-Log".to_string()),
            )),
        );
    }

    #[test]
    fn not_emoji() {
        let message = indoc!(
            "
            An example commit

            This is an example commit
            "
        );
        run_lint(
            message,
            &Some(Problem::new(
                ERROR.into(),
                HELP_MESSAGE.into(),
                Code::NotEmojiLog,
                &message.into(),
                Some(vec![("Not emoji log".to_string(), 0_usize, 17_usize)]),
                Some("https://github.com/ahmadawais/Emoji-Log".to_string()),
            )),
        );
    }

    use std::option::Option::None;

    use miette::{GraphicalReportHandler, GraphicalTheme, Report};

    #[test]
    fn formatting() {
        let message = indoc!(
            "
            An example commit

            This is an example commit
            "
        );
        let problem = lint(&CommitMessage::from(message.to_string()));
        let actual = fmt_report(&Report::new(problem.unwrap()));
        let expected = "\u{1b}]8;;https://github.com/ahmadawais/Emoji-Log\u{1b}\\NotEmojiLog (link)\u{1b}]8;;\u{1b}\\

  \u{d7} Your commit message isn't in emoji log style
   \u{256d}\u{2500}[1:1]
 1 \u{2502} An example commit
   \u{b7} \u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{252c}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}
   \u{b7}         \u{2570}\u{2500}\u{2500} Not emoji log
 2 \u{2502} 
   \u{2570}\u{2500}\u{2500}\u{2500}\u{2500}
  help: It's important to follow the emoji log style when creating your
        commit message. By using this style we can automatically generate
        changelogs.
        
        You can fix it using one of the prefixes:
        
        
        \u{1f4e6} NEW:
        \u{1f44c} IMPROVE:
        \u{1f41b} FIX:
        \u{1f4d6} DOC:
        \u{1f680} RELEASE:
        \u{1f916} TEST:
        \u{203c}\u{fe0f} BREAKING:
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
        GraphicalReportHandler::new_themed(GraphicalTheme::unicode_nocolor())
            .with_width(80)
            .render_report(&mut out, diag.as_ref())
            .unwrap();
        out
    }

    fn run_lint(message: &str, expected: &Option<Problem>) {
        let actual = &lint(&CommitMessage::from(message));
        assert_eq!(
            actual, expected,
            "Message {:?} should have returned {:?}, found {:?}",
            message, expected, actual
        );
    }
}
