use std::{
    collections::{BTreeMap, BTreeSet},
    convert::{TryFrom, TryInto},
};

use quickcheck::TestResult;

use crate::model::{
    lint::Lint::{
        BodyWiderThan72Characters,
        DuplicatedTrailers,
        JiraIssueKeyMissing,
        PivotalTrackerIdMissing,
        SubjectLongerThan72Characters,
        SubjectNotSeparateFromBody,
    },
    lints::Error,
    Lint,
    Lints,
};

#[allow(clippy::needless_pass_by_value)]
#[quickcheck]
fn it_returns_an_error_if_one_of_the_names_is_wrong(lints: Vec<String>) -> TestResult {
    if lints.is_empty() {
        return TestResult::discard();
    }

    let actual: Result<Lints, Error> = lints
        .iter()
        .map(std::borrow::Borrow::borrow)
        .collect::<Vec<&str>>()
        .try_into();

    TestResult::from_bool(actual.is_err())
}

#[allow(clippy::needless_pass_by_value)]
#[allow(unused_must_use)]
#[quickcheck]
fn no_lint_segfaults(lint: Lint, commit: String) -> TestResult {
    lint.lint(&commit.into());

    TestResult::passed()
}

#[test]
fn example_it_returns_an_error_if_one_of_the_names_is_wrong() {
    let lints = vec![
        "pivotal-tracker-id-missing",
        "broken",
        "jira-issue-key-missing",
    ];
    let actual: Result<Lints, Error> = lints.try_into();

    actual.unwrap_err();
}

#[quickcheck]
fn it_can_construct_itself_from_names(lints: Vec<Lint>) -> bool {
    let lint_names: Vec<&str> = lints.clone().into_iter().map(Lint::name).collect();

    let expected_lints = lints.into_iter().collect::<BTreeSet<Lint>>();
    let expected = Lints::new(expected_lints);

    let actual: Lints = lint_names.try_into().expect("Lints to have been parsed");

    expected == actual
}

#[test]
fn example_it_can_construct_itself_from_names() {
    let lints = vec!["pivotal-tracker-id-missing", "jira-issue-key-missing"];

    let mut expected_lints = BTreeSet::new();
    expected_lints.insert(PivotalTrackerIdMissing);
    expected_lints.insert(JiraIssueKeyMissing);

    let expected = Lints::new(expected_lints);
    let actual: Lints = lints.try_into().expect("Lints to have been parsed");

    assert_eq!(expected, actual);
}

#[quickcheck]
fn it_can_give_me_an_into_iterator(lint_vec: Vec<Lint>) -> bool {
    let lints = lint_vec.into_iter().collect::<BTreeSet<_>>();
    let input = Lints::new(lints.clone());

    let expected = lints.into_iter().collect::<Vec<_>>();
    let actual = input.into_iter().collect::<Vec<_>>();

    expected == actual
}

#[test]
fn example_it_can_give_me_an_into_iterator() {
    let mut lints = BTreeSet::new();
    lints.insert(PivotalTrackerIdMissing);
    lints.insert(JiraIssueKeyMissing);
    let input = Lints::new(lints);

    let expected = vec![PivotalTrackerIdMissing, JiraIssueKeyMissing];
    let actual = input.into_iter().collect::<Vec<_>>();

    assert_eq!(expected, actual);
}

#[quickcheck]
fn it_can_convert_into_a_vec(lint_vec: Vec<Lint>) -> bool {
    let lints = lint_vec.into_iter().collect::<BTreeSet<_>>();
    let input = Lints::new(lints.clone());

    let expected = lints.into_iter().collect::<Vec<_>>();
    let actual: Vec<_> = input.into();

    expected == actual
}

#[test]
fn example_it_can_convert_into_a_vec() {
    let mut lints = BTreeSet::new();
    lints.insert(PivotalTrackerIdMissing);
    lints.insert(JiraIssueKeyMissing);
    let input = Lints::new(lints);

    let expected = vec![PivotalTrackerIdMissing, JiraIssueKeyMissing];
    let actual: Vec<Lint> = input.into();

    assert_eq!(expected, actual);
}

#[quickcheck]
fn it_can_give_me_the_names(lints: BTreeSet<Lint>) -> bool {
    let lint_names: Vec<&str> = lints.clone().into_iter().map(Lint::name).collect();
    let actual = Lints::from(lints.into_iter().collect::<Vec<Lint>>()).names();

    lint_names == actual
}

#[test]
fn example_it_can_give_me_the_names() {
    let mut lints = BTreeSet::new();
    lints.insert(PivotalTrackerIdMissing);
    lints.insert(JiraIssueKeyMissing);
    let input = Lints::new(lints);

    let expected = vec![PivotalTrackerIdMissing.name(), JiraIssueKeyMissing.name()];
    let actual = input.names();

    assert_eq!(expected, actual);
}

#[quickcheck]
fn it_can_give_me_the_config_keys(lints: BTreeSet<Lint>) -> bool {
    let lint_names: Vec<String> = lints.clone().into_iter().map(Lint::config_key).collect();
    let actual = Lints::from(lints.into_iter().collect::<Vec<Lint>>()).config_keys();

    lint_names == actual
}

#[test]
fn example_it_can_give_me_the_config_keys() {
    let mut lints = BTreeSet::new();
    lints.insert(PivotalTrackerIdMissing);
    lints.insert(JiraIssueKeyMissing);
    let input = Lints::new(lints);

    let expected = vec![
        PivotalTrackerIdMissing.config_key(),
        JiraIssueKeyMissing.config_key(),
    ];
    let actual = input.config_keys();

    assert_eq!(expected, actual);
}

#[test]
fn can_get_all() {
    let actual = Lints::available();
    let lints = Lint::all_lints().collect();
    let expected = &Lints::new(lints);

    assert_eq!(
        expected, actual,
        "Expected all the lints to be {:?}, instead got {:?}",
        expected, actual
    );
}

#[test]
fn example_can_get_all() {
    let actual = Lints::available();
    let lints = Lint::all_lints().collect();
    let expected = &Lints::new(lints);

    assert_eq!(
        expected, actual,
        "Expected all the lints to be {:?}, instead got {:?}",
        expected, actual
    );
}

#[allow(clippy::needless_pass_by_value)]
#[quickcheck]
fn get_toml(expected: BTreeMap<Lint, bool>) -> bool {
    let toml = String::try_from(Lints::new(
        expected
            .iter()
            .filter(|(_, enabled)| **enabled)
            .map(|(lint, _)| *lint)
            .collect(),
    ))
    .expect("To be able to convert lints to toml");
    let full: BTreeMap<String, BTreeMap<String, BTreeMap<String, bool>>> =
        toml::from_str(toml.as_str()).unwrap();
    let actual: BTreeMap<Lint, bool> = full
        .get("mit")
        .and_then(|x| x.get("lint"))
        .expect("To have successfully removed the wrapping keys")
        .iter()
        .map(|(lint, enabled)| (Lint::try_from(lint.as_str()).unwrap(), *enabled))
        .collect();

    actual
        .iter()
        .map(|(actual_key, actual_enabled)| {
            expected
                .get(actual_key)
                .map_or(!*actual_enabled, |expected_enabled| {
                    expected_enabled == actual_enabled
                })
        })
        .all(|x| x)
}

#[test]
fn example_get_toml() {
    let mut lints_on = BTreeSet::new();
    lints_on.insert(DuplicatedTrailers);
    lints_on.insert(SubjectNotSeparateFromBody);
    lints_on.insert(SubjectLongerThan72Characters);
    lints_on.insert(BodyWiderThan72Characters);
    lints_on.insert(PivotalTrackerIdMissing);
    let actual = String::try_from(Lints::new(lints_on)).expect("Failed to serialise");
    let expected = "[mit.lint]
body-wider-than-72-characters = true
duplicated-trailers = true
github-id-missing = false
jira-issue-key-missing = false
not-conventional-commit = false
not-emoji-log = false
pivotal-tracker-id-missing = true
subject-line-ends-with-period = false
subject-line-not-capitalized = false
subject-longer-than-72-characters = true
subject-not-separated-from-body = true
";

    assert_eq!(
        expected, actual,
        "Expected the list of lint identifiers to be {:?}, instead got {:?}",
        expected, actual
    );
}

#[allow(clippy::needless_pass_by_value)]
#[quickcheck]
fn two_sets_of_lints_can_be_merged(
    set_a_lints: BTreeSet<Lint>,
    set_b_lints: BTreeSet<Lint>,
) -> bool {
    let set_a = Lints::new(set_a_lints.clone());
    let set_b = Lints::new(set_b_lints.clone());

    let actual = set_a.merge(&set_b);

    let expected = Lints::new(set_a_lints.union(&set_b_lints).copied().collect());

    expected == actual
}

#[test]
fn example_two_sets_of_lints_can_be_merged() {
    let mut set_a_lints = BTreeSet::new();
    set_a_lints.insert(PivotalTrackerIdMissing);

    let mut set_b_lints = BTreeSet::new();
    set_b_lints.insert(DuplicatedTrailers);

    let set_a = Lints::new(set_a_lints);
    let set_b = Lints::new(set_b_lints);

    let actual = set_a.merge(&set_b);

    let mut lints = BTreeSet::new();
    lints.insert(DuplicatedTrailers);
    lints.insert(PivotalTrackerIdMissing);
    let expected = Lints::new(lints);

    assert_eq!(
        expected, actual,
        "Expected the list of lint identifiers to be {:?}, instead got {:?}",
        expected, actual
    );
}

#[allow(clippy::needless_pass_by_value)]
#[quickcheck]
fn we_can_subtract_lints_from_the_lint_list(
    set_a_lints: BTreeSet<Lint>,
    set_b_lints: BTreeSet<Lint>,
) -> bool {
    let total = Lints::new(set_a_lints.union(&set_b_lints).copied().collect());
    let set_a = Lints::new(set_a_lints.difference(&set_b_lints).copied().collect());
    let expected = Lints::new(set_b_lints);

    let actual = total.subtract(&set_a);

    expected == actual
}

#[test]
fn example_we_can_subtract_lints_from_the_lint_list() {
    let mut set_a_lints = BTreeSet::new();
    set_a_lints.insert(JiraIssueKeyMissing);
    set_a_lints.insert(PivotalTrackerIdMissing);

    let mut set_b_lints = BTreeSet::new();
    set_b_lints.insert(DuplicatedTrailers);
    set_b_lints.insert(PivotalTrackerIdMissing);

    let set_a = Lints::new(set_a_lints);
    let set_b = Lints::new(set_b_lints);

    let actual = set_a.subtract(&set_b);

    let mut lints = BTreeSet::new();
    lints.insert(JiraIssueKeyMissing);
    let expected = Lints::new(lints);

    assert_eq!(
        expected, actual,
        "Expected the list of lint identifiers to be {:?}, instead got {:?}",
        expected, actual
    );
}

#[test]
fn example_when_merging_overlapping_does_not_lead_to_duplication() {
    let mut set_a_lints = BTreeSet::new();
    set_a_lints.insert(PivotalTrackerIdMissing);

    let mut set_b_lints = BTreeSet::new();
    set_b_lints.insert(DuplicatedTrailers);
    set_b_lints.insert(PivotalTrackerIdMissing);

    let set_a = Lints::new(set_a_lints);
    let set_b = Lints::new(set_b_lints);

    let actual = set_a.merge(&set_b);

    let mut lints = BTreeSet::new();
    lints.insert(DuplicatedTrailers);
    lints.insert(PivotalTrackerIdMissing);
    let expected = Lints::new(lints);

    assert_eq!(
        expected, actual,
        "Expected the list of lint identifiers to be {:?}, instead got {:?}",
        expected, actual
    );
}
