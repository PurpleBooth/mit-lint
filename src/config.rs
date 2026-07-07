//! Lint configuration structs
//!
//! Each lint has an associated configuration struct that controls its
//! behaviour. These structs implement [`serde::Serialize`] and
//! [`serde::Deserialize`], so they can be written to and read from a
//! configuration file (TOML, JSON, ...).

pub use crate::checks::body_wider_than_72_characters::BodyWidthConfig;
pub use crate::checks::duplicate_trailers::DuplicatedTrailersConfig;
pub use crate::checks::missing_github_id::GitHubIdConfig;
pub use crate::checks::missing_gitlab_id::GitLabIdConfig;
pub use crate::checks::missing_jira_issue_key::JiraIssueKeyConfig;
pub use crate::checks::missing_pivotal_tracker_id::PivotalTrackerIdConfig;
pub use crate::checks::not_conventional_commit::ConventionalCommitConfig;
pub use crate::checks::not_emoji_log::EmojiLogConfig;
pub use crate::checks::subject_line_ends_with_period::SubjectLineEndsWithPeriodConfig;
pub use crate::checks::subject_longer_than_72_characters::SubjectLengthConfig;
pub use crate::checks::subject_not_capitalized::SubjectNotCapitalizedConfig;
pub use crate::checks::subject_not_separate_from_body::SubjectNotSeparateFromBodyConfig;
