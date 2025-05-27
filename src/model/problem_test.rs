use std::option::Option::None;

use miette::Diagnostic;
use mit_commit::CommitMessage;

use crate::model::{Problem, code::Code};

#[test]
fn test_error_returns_correct_value() {
    let problem = Problem::new(
        "Some error".into(),
        String::new(),
        Code::NotConventionalCommit,
        &"".into(),
        None,
        None,
    );
    assert_eq!(problem.error(), "Some error");
}

#[test]
fn test_empty_commit_returns_no_labels() {
    let problem = Problem::new(
        String::new(),
        String::new(),
        Code::NotConventionalCommit,
        &"".into(),
        Some(vec![("String".to_string(), 10_usize, 20_usize)]),
        None,
    );
    assert!(problem.labels().is_none());
}

#[test]
fn test_empty_commit_returns_no_source_code() {
    let problem = Problem::new(
        String::new(),
        String::new(),
        Code::NotConventionalCommit,
        &"".into(),
        Some(vec![("String".to_string(), 10_usize, 20_usize)]),
        None,
    );
    assert!(problem.source_code().is_none());
}

#[allow(
    clippy::needless_pass_by_value,
    reason = "Cannot be passed by value, not supported by quickcheck"
)]
#[quickcheck]
fn test_error_matches_input(error: String) -> bool {
    let problem = Problem::new(
        error.clone(),
        String::new(),
        Code::NotConventionalCommit,
        &CommitMessage::from(""),
        None,
        None,
    );
    problem.error() == error
}

#[test]
fn test_tip_returns_correct_value() {
    let problem = Problem::new(
        String::new(),
        "Some tip".into(),
        Code::NotConventionalCommit,
        &"".into(),
        None,
        None,
    );
    assert_eq!(problem.tip(), "Some tip");
}

#[allow(
    clippy::needless_pass_by_value,
    reason = "Cannot be passed by value, not supported by quickcheck"
)]
#[quickcheck]
fn test_tip_matches_input(tip: String) -> bool {
    let problem = Problem::new(
        String::new(),
        tip.to_string(),
        Code::NotConventionalCommit,
        &"".into(),
        None,
        None,
    );
    problem.tip() == tip
}

#[test]
fn test_code_returns_correct_value() {
    let problem = Problem::new(
        String::new(),
        String::new(),
        Code::NotConventionalCommit,
        &"".into(),
        None,
        None,
    );
    assert_eq!(problem.code(), &Code::NotConventionalCommit);
}

#[quickcheck]
fn test_code_matches_input(code: Code) {
    let problem = Problem::new(String::new(), String::new(), code, &"".into(), None, None);

    assert_eq!(problem.code(), &code, "Code should match the input value");
}

#[test]
fn test_commit_message_returns_correct_value() {
    let problem = Problem::new(
        String::new(),
        String::new(),
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
fn test_commit_message_matches_input(message: String) {
    let problem = Problem::new(
        String::new(),
        String::new(),
        Code::NotConventionalCommit,
        &CommitMessage::from(message.clone()),
        None,
        None,
    );
    assert_eq!(
        problem.commit_message(),
        CommitMessage::from(message),
        "Commit message should match the input value"
    );
}

#[test]
fn test_labels_return_correct_values() {
    let problem = Problem::new(
        String::new(),
        String::new(),
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
fn test_labels_match_input_values(start: usize, offset: usize) {
    let problem = Problem::new(
        String::new(),
        String::new(),
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
