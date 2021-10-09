use std::{ops::Add, option::Option::None};

use mit_commit::CommitMessage;

use crate::model::{Code, Problem};

/// Canonical lint ID
pub const CONFIG: &str = "pivotal-tracker-id-missing";

/// Advice on how to correct the problem
const HELP_MESSAGE: &str =
    "It's important to add the ID because it allows code to be linked back to the stories it was \
done for, it can provide a chain of custody for code for audit purposes, and it can give \
future explorers of the codebase insight into the wider organisational need behind the \
change. We may also use it for automation purposes, like generating changelogs or \
notification emails.

You can fix this by adding the Id in one of the styles below to the commit message
[Delivers #12345678]
[fixes #12345678]
[finishes #12345678]
[#12345884 #12345678]
[#12345884,#12345678]
[#12345678],[#12345884]
This will address [#12345884]";

/// Description of the problem
const ERROR: &str = "Your commit message is missing a Pivotal Tracker ID";

lazy_static! {
    static ref RE: regex::Regex = regex::Regex::new(
        r"(?i)\[(((finish|fix)(ed|es)?|complete[ds]?|deliver(s|ed)?) )?#\d+([, ]#\d+)*]"
    )
    .unwrap();
}

pub fn lint(commit_message: &CommitMessage) -> Option<Problem> {
    if commit_message.matches_pattern(&*RE) {
        None
    } else {
        let commit_text = String::from(commit_message.clone());
        let last_line_location = commit_text
            .trim_end()
            .rfind('\n')
            .unwrap_or_default()
            .add(1);
        Some(Problem::new(
            ERROR.into(),
            HELP_MESSAGE.into(),
            Code::PivotalTrackerIdMissing,
            commit_message,
            Some(vec![(
                "No Pivotal Tracker ID".to_string(),
                last_line_location,
                commit_text.len().saturating_sub(last_line_location),
            )]),
            Some("https://www.pivotaltracker.com/help/api?version=v5#Tracker_Updates_in_SCM_Post_Commit_Hooks".to_string()),
        ))
    }
}

#[cfg(test)]
mod tests_has_missing_pivotal_tracker_id {
    #![allow(clippy::wildcard_imports)]

    use std::option::Option::None;

    use miette::{GraphicalReportHandler, GraphicalTheme, Report};

    use super::*;
    use crate::model::{Code, Problem};

    #[test]
    fn with_id() {
        test_has_missing_pivotal_tracker_id(
            "An example commit

This is an example commit

[#12345678]
# Some comment
",
            &None,
        );
    }

    fn test_has_missing_pivotal_tracker_id(message: &str, expected: &Option<Problem>) {
        let actual = &lint(&CommitMessage::from(message));
        assert_eq!(
            actual, expected,
            "Message {:?} should have returned {:?}, found {:?}",
            message, expected, actual
        );
    }

    #[test]
    fn multiple_ids() {
        test_has_missing_pivotal_tracker_id(
            "An example commit

This is an example commit

[#12345678,#87654321]
# some comment
",
            &None,
        );
        test_has_missing_pivotal_tracker_id(
            "An example commit

This is an example commit

[#12345678,#87654321,#11223344]
# some comment
",
            &None,
        );
        test_has_missing_pivotal_tracker_id(
            "An example commit

This is an example commit

[#12345678 #87654321 #11223344]
# some comment
",
            &None,
        );
    }

    #[test]
    fn id_with_fixed_state_change() {
        test_has_missing_pivotal_tracker_id(
            "An example commit

This is an example commit

[fix #12345678]
# some comment
",
            &None,
        );
        test_has_missing_pivotal_tracker_id(
            "An example commit

This is an example commit

[FIX #12345678]
# some comment
",
            &None,
        );
        test_has_missing_pivotal_tracker_id(
            "An example commit

This is an example commit

[Fix #12345678]
# some comment
",
            &None,
        );
        test_has_missing_pivotal_tracker_id(
            "An example commit

This is an example commit

[fixed #12345678]
# some comment
",
            &None,
        );
        test_has_missing_pivotal_tracker_id(
            "An example commit

This is an example commit

[fixes #12345678]
# some comment
",
            &None,
        );
    }

    #[test]
    fn id_with_complete_state_change() {
        test_has_missing_pivotal_tracker_id(
            "An example commit

This is an example commit

[complete #12345678]
# some comment
",
            &None,
        );

        test_has_missing_pivotal_tracker_id(
            "An example commit

This is an example commit

[completed #12345678]
# some comment
",
            &None,
        );

        test_has_missing_pivotal_tracker_id(
            "An example commit

This is an example commit

[Completed #12345678]
# some comment
",
            &None,
        );

        test_has_missing_pivotal_tracker_id(
            "An example commit

This is an example commit

[completes #12345678]
# some comment
",
            &None,
        );
    }

    #[test]
    fn id_with_finished_state_change() {
        test_has_missing_pivotal_tracker_id(
            "An example commit

This is an example commit

[finish #12345678]
# some comment
",
            &None,
        );

        test_has_missing_pivotal_tracker_id(
            "An example commit

This is an example commit

[finished #12345678]
# some comment
",
            &None,
        );
        test_has_missing_pivotal_tracker_id(
            "An example commit

This is an example commit

[finishes #12345678]
# some comment
",
            &None,
        );
    }

    #[test]
    fn id_with_delivered_state_change() {
        test_has_missing_pivotal_tracker_id(
            "An example commit

This is an example commit

[deliver #12345678]
# some comment
",
            &None,
        );

        test_has_missing_pivotal_tracker_id(
            "An example commit

This is an example commit

[delivered #12345678]
# some comment
",
            &None,
        );
        test_has_missing_pivotal_tracker_id(
            "An example commit

This is an example commit

[delivers #12345678]
# some comment
",
            &None,
        );
    }

    #[test]
    fn id_with_state_change_and_multiple_ids() {
        test_has_missing_pivotal_tracker_id(
            "An example commit

This is an example commit

[fix #12345678 #12345678]
# some comment
",
            &None,
        );
    }

    #[test]
    fn id_with_prefixed_text() {
        test_has_missing_pivotal_tracker_id(
            "An example commit

This is an example commit

Finally [fix #12345678 #12345678]
",
            &None,
        );
    }

    #[test]
    fn invalid_state_change() {
        let message = "An example commit

This is an example commit

[fake #12345678]
# some comment
";
        test_has_missing_pivotal_tracker_id(
            message,
            &Some(Problem::new(
                ERROR.into(),
                HELP_MESSAGE.into(),
                Code::PivotalTrackerIdMissing,
                &message.into(),
                Some(vec![("No Pivotal Tracker ID".to_string(), 63, 15)]),
                Some("https://www.pivotaltracker.com/help/api?version=v5#Tracker_Updates_in_SCM_Post_Commit_Hooks".parse().unwrap()),
            )),
        );
    }

    #[test]
    fn missing_id_with_square_brackets() {
        let message_1 = "An example commit

This is an example commit
";
        test_has_missing_pivotal_tracker_id(
            message_1,
            &Some(Problem::new(
                ERROR.into(),
                HELP_MESSAGE.into(),
                Code::PivotalTrackerIdMissing,
                &message_1.into(),
                Some(vec![("No Pivotal Tracker ID".to_string(), 19, 26)]),
                Some("https://www.pivotaltracker.com/help/api?version=v5#Tracker_Updates_in_SCM_Post_Commit_Hooks".parse().unwrap()),
            )),
        );

        let message_2 = "An example commit

This is an example commit

[#]
# some comment
";
        test_has_missing_pivotal_tracker_id(
            message_2,
            &Some(Problem::new(
                ERROR.into(),
                HELP_MESSAGE.into(),
                Code::PivotalTrackerIdMissing,
                &message_2.into(),
                Some(vec![("No Pivotal Tracker ID".to_string(), 50, 15)]),
                Some("https://www.pivotaltracker.com/help/api?version=v5#Tracker_Updates_in_SCM_Post_Commit_Hooks".parse().unwrap()),
            )),
        );
    }

    #[test]
    fn formatting() {
        let message = "An example commit

This is an example commit
";
        let problem = lint(&CommitMessage::from(message.to_string()));
        let actual = fmt_report(&Report::new(problem.unwrap()));
        let expected = "PivotalTrackerIdMissing (https://www.pivotaltracker.com/help/api?version=v5#Tracker_Updates_in_SCM_Post_Commit_Hooks)

  x Your commit message is missing a Pivotal Tracker ID
   ,-[2:1]
 2 | 
 3 | This is an example commit
   : ^^^^^^^^^^^^^|^^^^^^^^^^^^
   :              `-- No Pivotal Tracker ID
   `----
  help: It's important to add the ID because it allows code to be linked
        back to the stories it was done for, it can provide a chain
        of custody for code for audit purposes, and it can give future
        explorers of the codebase insight into the wider organisational need
        behind the change. We may also use it for automation purposes, like
        generating changelogs or notification emails.
        
        You can fix this by adding the Id in one of the styles below to the
        commit message
        [Delivers #12345678]
        [fixes #12345678]
        [finishes #12345678]
        [#12345884 #12345678]
        [#12345884,#12345678]
        [#12345678],[#12345884]
        This will address [#12345884]
"        .to_string();
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
}
