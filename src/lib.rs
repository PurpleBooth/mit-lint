//! A set of lints to use with mit-commit
//!
//! # Examples
//!
//! ``` rust
//! use mit_commit::CommitMessage;
//! use mit_lint::{Code, lint, Problem, Lints, Lint};
//! use std::option::Option::None;
//!
//! let message:String = "x".repeat(73).into();
//! let expected = vec![Problem::new(
//!     "Your subject is longer than 72 characters".into(),
//!     "It's important to keep the subject of the commit less than 72 characters because when you look at the git log, that's where it truncates the message. This means that people won't get the entirety of the information in your commit.\n\nPlease keep the subject line 72 characters or under"
//!         .into(),
//!     Code::SubjectLongerThan72Characters,&message.clone().into(),Some(vec![(String::from("Too long"), 73, 1)]),None,
//! )];
//! let actual = lint(&CommitMessage::from(message), Lints::new(vec![Lint::SubjectLongerThan72Characters].into_iter().collect()));
//! assert_eq!(
//!     actual, expected,
//!     "Expected {:?}, found {:?}",
//!     expected, actual
//! );
//! ```

#[macro_use]
extern crate lazy_static;

pub use cmd::{async_lint, lint};
pub use model::{Code, Error, Lint, LintError, Lints, Problem, CONFIG_KEY_PREFIX};

mod checks;
mod cmd;
mod model;
