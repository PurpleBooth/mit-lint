#[macro_use]
extern crate lazy_static;

pub use cmd::lint;
pub use model::CONFIG_KEY_PREFIX;
pub use model::{Code, Error, Lint, LintError, Lints, Problem};

mod checks;
mod cmd;
mod model;
