//! Round-trip serialization tests for lint configuration structs.
//!
//! Each config struct should be serialisable to TOML (and JSON) and
//! deserialisable back to an equal value.

use std::collections::HashSet;

use mit_lint::{
    BodyWidthConfig, ConventionalCommitConfig, DuplicatedTrailersConfig, EmojiLogConfig,
    GitHubIdConfig, GitLabIdConfig, JiraIssueKeyConfig, PivotalTrackerIdConfig,
    SubjectLengthConfig, SubjectLineEndsWithPeriodConfig, SubjectNotCapitalizedConfig,
    SubjectNotSeparateFromBodyConfig,
};

fn round_trip_toml<T>(original: &T)
where
    T: serde::Serialize + serde::de::DeserializeOwned + PartialEq + std::fmt::Debug,
{
    let toml = toml::to_string(original).expect("Failed to serialise to TOML");
    let deserialised: T = toml::from_str(&toml).expect("Failed to deserialise from TOML");
    assert_eq!(
        original, &deserialised,
        "Round-trip through TOML failed.\nTOML:\n{toml}"
    );
}

fn round_trip_json<T>(original: &T)
where
    T: serde::Serialize + serde::de::DeserializeOwned + PartialEq + std::fmt::Debug,
{
    let json = serde_json::to_string(original).expect("Failed to serialise to JSON");
    let deserialised: T = serde_json::from_str(&json).expect("Failed to deserialise from JSON");
    assert_eq!(
        original, &deserialised,
        "Round-trip through JSON failed.\nJSON:\n{json}"
    );
}

// --- Unit structs ---
// Unit structs (no fields) cannot be serialised to TOML (which has no
// representation for a bare unit), so we round-trip them through JSON only.

#[test]
fn round_trip_jira_issue_key_config() {
    let config = JiraIssueKeyConfig;
    round_trip_json(&config);
}

#[test]
fn round_trip_pivotal_tracker_id_config() {
    let config = PivotalTrackerIdConfig;
    round_trip_json(&config);
}

#[test]
fn round_trip_emoji_log_config() {
    let config = EmojiLogConfig;
    round_trip_json(&config);
}

#[test]
fn round_trip_subject_not_capitalized_config() {
    let config = SubjectNotCapitalizedConfig;
    round_trip_json(&config);
}

#[test]
fn round_trip_subject_not_separate_from_body_config() {
    let config = SubjectNotSeparateFromBodyConfig;
    round_trip_json(&config);
}

#[test]
fn round_trip_subject_line_ends_with_period_config() {
    let config = SubjectLineEndsWithPeriodConfig;
    round_trip_json(&config);
}

// --- Structs with simple fields ---

#[test]
fn round_trip_subject_length_config_default() {
    let config = SubjectLengthConfig::default();
    round_trip_toml(&config);
    round_trip_json(&config);
}

#[test]
fn round_trip_subject_length_config_custom() {
    let config = SubjectLengthConfig {
        character_limit: 50,
    };
    round_trip_toml(&config);
    round_trip_json(&config);
}

#[test]
fn round_trip_body_width_config_default() {
    let config = BodyWidthConfig::default();
    round_trip_toml(&config);
    round_trip_json(&config);
}

#[test]
fn round_trip_body_width_config_custom() {
    let config = BodyWidthConfig {
        character_limit: 100,
    };
    round_trip_toml(&config);
    round_trip_json(&config);
}

#[test]
fn round_trip_duplicated_trailers_config_default() {
    let config = DuplicatedTrailersConfig::default();
    round_trip_toml(&config);
    round_trip_json(&config);
}

#[test]
fn round_trip_duplicated_trailers_config_custom() {
    let config = DuplicatedTrailersConfig {
        trailers_to_check: vec!["Custom-Trailer".to_string(), "Another-Trailer".to_string()],
    };
    round_trip_toml(&config);
    round_trip_json(&config);
}

// --- Structs with Regex fields ---

#[test]
fn round_trip_github_id_config_default() {
    let config = GitHubIdConfig::default();
    round_trip_toml(&config);
    round_trip_json(&config);
}

#[test]
fn round_trip_gitlab_id_config_default() {
    let config = GitLabIdConfig::default();
    round_trip_toml(&config);
    round_trip_json(&config);
}

// --- Structs with Option<HashSet<String>> fields ---

#[test]
fn round_trip_conventional_commit_config_default() {
    let config = ConventionalCommitConfig::default();
    round_trip_toml(&config);
    round_trip_json(&config);
}

#[test]
fn round_trip_conventional_commit_config_with_types() {
    let mut types = HashSet::new();
    types.insert("feat".to_string());
    types.insert("fix".to_string());
    let config = ConventionalCommitConfig {
        allowed_types: Some(types),
        allowed_scopes: None,
    };
    round_trip_toml(&config);
    round_trip_json(&config);
}

// --- TOML output verification ---

#[test]
fn subject_length_config_toml_output() {
    let config = SubjectLengthConfig {
        character_limit: 72,
    };
    let toml = toml::to_string(&config).expect("Failed to serialise to TOML");
    assert!(
        toml.contains("character_limit = 72"),
        "Expected character_limit in TOML output, got:\n{toml}"
    );
}

#[test]
fn duplicated_trailers_config_toml_output() {
    let config = DuplicatedTrailersConfig {
        trailers_to_check: vec!["Signed-off-by".to_string()],
    };
    let toml = toml::to_string(&config).expect("Failed to serialise to TOML");
    assert!(
        toml.contains("\"Signed-off-by\""),
        "Expected trailer name in TOML output, got:\n{toml}"
    );
}
