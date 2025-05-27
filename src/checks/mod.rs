pub mod body_wider_than_72_characters;
pub mod duplicate_trailers;
#[cfg(test)]
mod duplicate_trailers_test;
pub mod missing_github_id;
#[cfg(test)]
mod missing_github_id_test;
pub mod missing_jira_issue_key;
#[cfg(test)]
mod missing_jira_issue_key_test;
pub mod missing_pivotal_tracker_id;
#[cfg(test)]
mod missing_pivotal_tracker_id_test;
pub mod not_conventional_commit;
#[cfg(test)]
mod not_conventional_commit_test;
pub mod not_emoji_log;
#[cfg(test)]
mod not_emoji_log_test;
pub mod subject_line_ends_with_period;
#[cfg(test)]
mod subject_line_ends_with_period_test;
pub mod subject_longer_than_72_characters;
#[cfg(test)]
mod subject_longer_than_72_characters_test;
pub mod subject_not_capitalized;
#[cfg(test)]
mod subject_not_capitalized_test;
pub mod subject_not_separate_from_body;
#[cfg(test)]
mod subject_not_separate_from_body_test;
