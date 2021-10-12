use std::option::Option::None;

use mit_commit::CommitMessage;
use quickcheck::{Arbitrary, Gen};
use strum::IntoEnumIterator;
use strum_macros::EnumIter;

use crate::model::{Code, Problem};

/// Canonical lint ID
pub const CONFIG: &str = "not-emoji-log";

/// Advice on how to correct the problem
pub const HELP_MESSAGE: &str = "It's important to follow the emoji log style when creating your commit message. By using this \
style we can automatically generate changelogs.

You can fix it using one of the prefixes:


\u{1f4e6} NEW:
\u{1f44c} IMPROVE:
\u{1f41b} FIX:
\u{1f4d6} DOC:
\u{1f680} RELEASE:
\u{1f916} TEST:
\u{203c}\u{fe0f} BREAKING:";
/// Description of the problem
pub const ERROR: &str = "Your commit message isn't in emoji log style";

pub fn lint(commit_message: &CommitMessage) -> Option<Problem> {
    if Prefix::iter().any(|x| {
        commit_message
            .get_subject()
            .to_string()
            .starts_with(&String::from(x))
    }) {
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
                commit_text.lines().next().map(str::len).unwrap_or_default(),
            )]),
            Some("https://github.com/ahmadawais/Emoji-Log".to_string()),
        ))
    }
}

#[derive(Clone, Copy, Eq, PartialEq, Debug, EnumIter)]
pub enum Prefix {
    Fix,
    New,
    Improve,
    Doc,
    Release,
    Test,
    Breaking,
}

impl From<Prefix> for String {
    fn from(input: Prefix) -> Self {
        match input {
            Prefix::Fix => Self::from("\u{1f41b} FIX: "),
            Prefix::New => Self::from("\u{1f4e6} NEW: "),
            Prefix::Improve => Self::from("\u{1f44c} IMPROVE: "),
            Prefix::Doc => Self::from("\u{1f4d6} DOC: "),
            Prefix::Release => Self::from("\u{1f680} RELEASE: "),
            Prefix::Test => Self::from("\u{1f916} TEST: "),
            Prefix::Breaking => Self::from("\u{203c}\u{fe0f} BREAKING: "),
        }
    }
}

impl Arbitrary for Prefix {
    fn arbitrary(g: &mut Gen) -> Self {
        *g.choose(&Self::iter().collect::<Vec<_>>()).unwrap()
    }

    fn shrink(&self) -> Box<dyn Iterator<Item = Self>> {
        let mut prefixes = Self::iter();

        let index = Self::iter().position(|other| self.eq(&other));

        match index {
            None | Some(0) => quickcheck::empty_shrinker(),
            Some(index) => prefixes
                .nth(index - 1)
                .map_or(quickcheck::empty_shrinker(), |item| {
                    quickcheck::single_shrinker(item)
                }),
        }
    }
}
