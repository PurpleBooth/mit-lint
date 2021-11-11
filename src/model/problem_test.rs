use std::option::Option::None;

use miette::Diagnostic;
use mit_commit::CommitMessage;

use crate::model::{code::Code, Problem};

#[test]
fn examples_has_error() {
    let problem = Problem::new(
        "Some error".into(),
        "".into(),
        Code::NotConventionalCommit,
        &"".into(),
        None,
        None,
    );
    assert_eq!(problem.error(), "Some error");
}

#[test]
fn labels_are_none_if_commit_empty() {
    let problem = Problem::new(
        "".into(),
        "".into(),
        Code::NotConventionalCommit,
        &"".into(),
        Some(vec![("String".to_string(), 10_usize, 20_usize)]),
        None,
    );
    assert!(problem.labels().is_none());
}

#[test]
fn commit_message_is_none_when_it_is_empty() {
    let problem = Problem::new(
        "".into(),
        "".into(),
        Code::NotConventionalCommit,
        &"".into(),
        Some(vec![("String".to_string(), 10_usize, 20_usize)]),
        None,
    );
    assert!(problem.source_code().is_none());
}

#[allow(clippy::needless_pass_by_value)]
#[quickcheck]
fn test_has_error(error: String) -> bool {
    let problem = Problem::new(
        error.clone(),
        "".into(),
        Code::NotConventionalCommit,
        &"".into(),
        None,
        None,
    );
    problem.error() == error
}

#[test]
fn examples_has_has_tip() {
    let problem = Problem::new(
        "".into(),
        "Some tip".into(),
        Code::NotConventionalCommit,
        &"".into(),
        None,
        None,
    );
    assert_eq!(problem.tip(), "Some tip");
}

#[allow(clippy::needless_pass_by_value)]
#[quickcheck]
fn test_has_has_tip(tip: String) -> bool {
    let problem = Problem::new(
        "".into(),
        tip.clone(),
        Code::NotConventionalCommit,
        &"".into(),
        None,
        None,
    );
    problem.tip() == tip
}

#[test]
fn examples_has_has_code() {
    let problem = Problem::new(
        "".into(),
        "".into(),
        Code::NotConventionalCommit,
        &"".into(),
        None,
        None,
    );
    assert_eq!(problem.code(), &Code::NotConventionalCommit);
}

#[quickcheck]
fn test_has_has_code(code: Code) {
    let problem = Problem::new("".into(), "".into(), code, &"".into(), None, None);
    let _ = problem.code() == &code;
}

#[test]
fn examples_it_contains_the_triggering_message() {
    let problem = Problem::new(
        "".into(),
        "".into(),
        Code::NotConventionalCommit,
        &CommitMessage::from("Commit message"),
        None,
        None,
    );
    assert_eq!(
        problem.commit_message(),
        CommitMessage::from("Commit message")
    );
}

#[quickcheck]
fn test_it_contains_the_triggering_message(message: String) {
    let problem = Problem::new(
        "".into(),
        "".into(),
        Code::NotConventionalCommit,
        &CommitMessage::from(message.clone()),
        None,
        None,
    );
    assert_eq!(problem.commit_message(), CommitMessage::from(message));
}

#[test]
fn examples_it_contains_the_labels() {
    let problem = Problem::new(
        "".into(),
        "".into(),
        Code::NotConventionalCommit,
        &CommitMessage::from("Commit message"),
        Some(vec![("String".to_string(), 10_usize, 20_usize)]),
        None,
    );
    assert_eq!(
        problem
            .labels()
            .unwrap()
            .map(|x| (x.label().unwrap().to_string(), x.offset(), x.len()))
            .collect::<Vec<_>>(),
        vec![("String".to_string(), 10_usize, 20_usize)]
    );
}
#[quickcheck]
fn test_it_contains_the_labels(start: usize, offset: usize) {
    let problem = Problem::new(
        "".into(),
        "".into(),
        Code::NotConventionalCommit,
        &CommitMessage::from("Commit message"),
        Some(vec![("String".to_string(), start, offset)]),
        None,
    );
    assert_eq!(
        problem
            .labels()
            .unwrap()
            .map(|x| (x.label().unwrap().to_string(), x.offset(), x.len()))
            .collect::<Vec<_>>(),
        vec![("String".to_string(), start, offset)]
    );
}
