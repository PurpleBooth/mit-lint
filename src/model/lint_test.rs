use std::convert::TryInto;

use crate::model::Lint;

#[quickcheck]
fn it_is_convertible_to_string(expected: Lint) -> bool {
    let lint: String = expected.into();
    expected.name() == lint
}

#[quickcheck]
fn it_can_be_created_from_string(expected: Lint) -> bool {
    let lint: Lint = expected.name().try_into().unwrap();
    expected == lint
}

#[quickcheck]
fn it_is_printable(lint: Lint) -> bool {
    lint.name() == format!("{}", lint)
}

#[quickcheck]
fn i_can_get_all_the_lints(lint: Lint) -> bool {
    Lint::all_lints().any(|x| x == lint)
}

#[test]
fn example_it_is_convertible_to_string() {
    let string: String = Lint::PivotalTrackerIdMissing.into();
    assert_eq!("pivotal-tracker-id-missing".to_string(), string);
}

#[test]
fn example_it_can_be_created_from_string() {
    let lint: Lint = "pivotal-tracker-id-missing".try_into().unwrap();
    assert_eq!(Lint::PivotalTrackerIdMissing, lint);
}

#[test]
fn example_it_is_printable() {
    assert_eq!(
        "pivotal-tracker-id-missing",
        &format!("{}", Lint::PivotalTrackerIdMissing)
    );
}

#[test]
fn example_i_can_get_all_the_lints() {
    let all: Vec<Lint> = Lint::all_lints().collect();
    assert_eq!(
        all,
        vec![
            Lint::DuplicatedTrailers,
            Lint::PivotalTrackerIdMissing,
            Lint::JiraIssueKeyMissing,
            Lint::SubjectNotSeparateFromBody,
            Lint::GitHubIdMissing,
            Lint::SubjectLongerThan72Characters,
            Lint::SubjectNotCapitalized,
            Lint::SubjectEndsWithPeriod,
            Lint::BodyWiderThan72Characters,
            Lint::NotConventionalCommit,
            Lint::NotEmojiLog,
        ]
    );
}

#[test]
fn example_i_can_get_if_a_lint_is_enabled_by_default() {
    assert!(Lint::DuplicatedTrailers.enabled_by_default());
    assert!(!Lint::PivotalTrackerIdMissing.enabled_by_default());
    assert!(!Lint::JiraIssueKeyMissing.enabled_by_default());
    assert!(Lint::SubjectNotSeparateFromBody.enabled_by_default());
    assert!(!Lint::GitHubIdMissing.enabled_by_default());
}
