use std::{ops::Add, option::Option::None};

use mit_commit::CommitMessage;

use crate::model::{Code, Problem};

/// Canonical lint ID
pub const CONFIG: &str = "pivotal-tracker-id-missing";

/// Advice on how to correct the problem
pub const HELP_MESSAGE: &str =
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
pub const ERROR: &str = "Your commit message is missing a Pivotal Tracker ID";

lazy_static! {
    static ref RE: regex::Regex = regex::Regex::new(
        r"(?i)\[(((finish|fix)(ed|es)?|complete[ds]?|deliver(s|ed)?) )?#\d+([, ]#\d+)*]"
    )
    .unwrap();
}

pub fn lint(commit_message: &CommitMessage<'_>) -> Option<Problem> {
    if commit_message.matches_pattern(&RE) {
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
                commit_text.len().saturating_sub(last_line_location+1),
            )]),
            Some("https://www.pivotaltracker.com/help/api?version=v5#Tracker_Updates_in_SCM_Post_Commit_Hooks".to_string()),
        ))
    }
}
