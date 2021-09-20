#[macro_use]
extern crate lazy_static;

pub use cmd::lint;
pub use model::{Code, Error, Lint, LintError, Lints, Problem, CONFIG_KEY_PREFIX};

mod checks;
mod cmd;
mod model;
