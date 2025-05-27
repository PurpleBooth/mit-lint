pub use code::Code;
pub use lint::{CONFIG_KEY_PREFIX, Error as LintError, Lint};
pub use lints::{Error, Lints};
pub use problem::Problem;

mod code;
mod lint;
#[cfg(test)]
mod lint_test;
mod lints;
#[cfg(test)]
mod lints_test;
mod problem;
#[cfg(test)]
mod problem_test;
