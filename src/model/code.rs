use quickcheck::{Arbitrary, Gen};
use strum_macros::EnumIter;

/// Error codes for lints that have failed
///
/// Useful for exit codes and other user facing things
#[derive(Clone, Copy, Debug, PartialEq, Eq, EnumIter)]
#[repr(i32)]
pub enum Code {
    InitialNotMatchedToAuthor = 3,
    UnparsableAuthorFile,
    StaleAuthor,
    DuplicatedTrailers,
    PivotalTrackerIdMissing,
    JiraIssueKeyMissing,
    GitHubIdMissing,
    SubjectNotSeparateFromBody,
    SubjectLongerThan72Characters,
    SubjectNotCapitalized,
    SubjectEndsWithPeriod,
    BodyWiderThan72Characters,
    NotConventionalCommit,
    NotEmojiLog,
}

impl Arbitrary for Code {
    fn arbitrary(g: &mut Gen) -> Self {
        *g.choose(&Self::get_codes()).unwrap()
    }

    fn shrink(&self) -> Box<dyn Iterator<Item = Self>> {
        let codes = Self::get_codes();

        let index = codes.iter().position(|other| self.eq(other));

        match index {
            None | Some(0) => quickcheck::empty_shrinker(),
            Some(index) => codes
                .get(index - 1)
                .map_or(quickcheck::empty_shrinker(), |item| {
                    quickcheck::single_shrinker(*item)
                }),
        }
    }
}

impl Code {
    const fn get_codes() -> [Self; 14] {
        [
            Self::InitialNotMatchedToAuthor,
            Self::UnparsableAuthorFile,
            Self::StaleAuthor,
            Self::DuplicatedTrailers,
            Self::PivotalTrackerIdMissing,
            Self::JiraIssueKeyMissing,
            Self::GitHubIdMissing,
            Self::SubjectNotSeparateFromBody,
            Self::SubjectLongerThan72Characters,
            Self::SubjectNotCapitalized,
            Self::SubjectEndsWithPeriod,
            Self::BodyWiderThan72Characters,
            Self::NotConventionalCommit,
            Self::NotEmojiLog,
        ]
    }
}
