#[derive(Clone, Copy, Debug, PartialEq, Eq)]
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
