use std::{
    collections::{BTreeMap, BTreeSet},
    convert::TryFrom,
    vec::IntoIter,
};

use miette::Diagnostic;
use thiserror::Error;

use crate::model::{lint, Lint};

/// A collection of lints
#[derive(Debug, Eq, PartialEq, Clone)]
pub struct Lints {
    lints: BTreeSet<Lint>,
}

lazy_static! {
    /// All the available lints
    static ref AVAILABLE: Lints = {
        let set = Lint::all_lints().collect();
        Lints::new(set)
    };
}

impl Lints {
    /// Create a new lint
    ///
    /// # Examples
    ///
    /// ```rust
    /// use std::collections::BTreeSet;
    ///
    /// use mit_lint::Lints;
    /// Lints::new(BTreeSet::new());
    /// ```
    #[must_use]
    pub const fn new(lints: BTreeSet<Lint>) -> Self {
        Self { lints }
    }

    /// Get the available lints
    ///
    /// # Examples
    ///
    /// ```rust
    /// use mit_lint::{Lint, Lints};
    ///
    /// let lints = Lints::available().clone();
    /// assert!(lints.into_iter().count() > 0);
    /// ```
    #[must_use]
    pub fn available() -> &'static Self {
        &AVAILABLE
    }

    /// Get all the names of these lints
    ///
    /// # Examples
    ///
    /// ```rust
    /// use mit_lint::{Lint, Lints};
    ///
    /// let names = Lints::available().clone().names();
    /// assert!(names.contains(&Lint::SubjectNotSeparateFromBody.name()));
    /// ```
    #[must_use]
    pub fn names(self) -> Vec<&'static str> {
        self.lints.iter().map(|lint| lint.name()).collect()
    }

    /// Get all the config keys of these lints
    ///
    /// # Examples
    ///
    /// ```rust
    /// use mit_lint::{Lint, Lints};
    ///
    /// let names = Lints::available().clone().config_keys();
    /// assert!(names.contains(&Lint::SubjectNotSeparateFromBody.config_key()));
    /// ```
    #[must_use]
    pub fn config_keys(self) -> Vec<String> {
        self.lints.iter().map(|lint| lint.config_key()).collect()
    }

    /// Create the union of two lints
    ///
    /// # Examples
    ///
    /// ```rust
    /// use mit_lint::{Lint, Lints};
    ///
    /// let to_add = Lints::new(vec![Lint::NotEmojiLog].into_iter().collect());
    /// let actual = Lints::available().clone().merge(&to_add).names();
    /// assert!(actual.contains(&Lint::NotEmojiLog.name()));
    /// ```
    #[must_use]
    pub fn merge(&self, other: &Self) -> Self {
        Self::new(self.lints.union(&other.lints).copied().collect())
    }

    /// Get the lints that are in self, but not in other
    ///
    /// # Examples
    ///
    /// ```rust
    /// use mit_lint::{Lint, Lints};
    ///
    /// let to_remove = Lints::new(vec![Lint::SubjectNotSeparateFromBody].into_iter().collect());
    /// let actual = Lints::available().clone().subtract(&to_remove).names();
    /// assert!(!actual.contains(&Lint::SubjectNotSeparateFromBody.name()));
    /// ```
    #[must_use]
    pub fn subtract(&self, other: &Self) -> Self {
        Self::new(self.lints.difference(&other.lints).copied().collect())
    }
}

impl std::iter::IntoIterator for Lints {
    type IntoIter = IntoIter<Lint>;
    type Item = Lint;

    fn into_iter(self) -> Self::IntoIter {
        self.lints.into_iter().collect::<Vec<_>>().into_iter()
    }
}

impl TryFrom<Lints> for String {
    type Error = Error;

    fn try_from(lints: Lints) -> Result<Self, Self::Error> {
        let enabled: Vec<_> = lints.into();

        let config: BTreeMap<Self, bool> = Lint::all_lints()
            .map(|x| (x, enabled.contains(&x)))
            .fold(BTreeMap::new(), |mut acc, (lint, state)| {
                acc.insert(lint.to_string(), state);
                acc
            });

        let mut inner: BTreeMap<Self, BTreeMap<Self, bool>> = BTreeMap::new();
        inner.insert("lint".into(), config);
        let mut output: BTreeMap<Self, BTreeMap<Self, BTreeMap<Self, bool>>> = BTreeMap::new();
        output.insert("mit".into(), inner);

        Ok(toml::to_string(&output)?)
    }
}

impl From<Vec<Lint>> for Lints {
    fn from(lints: Vec<Lint>) -> Self {
        Self::new(lints.into_iter().collect())
    }
}

impl From<Lints> for Vec<Lint> {
    fn from(lints: Lints) -> Self {
        lints.into_iter().collect()
    }
}

impl TryFrom<Vec<&str>> for Lints {
    type Error = Error;

    fn try_from(value: Vec<&str>) -> Result<Self, Self::Error> {
        let lints = value
            .into_iter()
            .try_fold(
                vec![],
                |lints: Vec<Lint>, item_name| -> Result<Vec<Lint>, Error> {
                    let lint = Lint::try_from(item_name)?;

                    Ok(vec![lints, vec![lint]].concat())
                },
            )
            .map(Vec::into_iter)?;

        Ok(Self::new(lints.collect()))
    }
}

#[derive(Error, Debug, Diagnostic)]
pub enum Error {
    #[error(transparent)]
    #[diagnostic(transparent)]
    LintNameUnknown(#[from] lint::Error),
    #[error("Failed to parse lint config file: {0}")]
    #[diagnostic(
        code(mit_lint::model::lints::error::toml_parse),
        url(docsrs),
        help("is it valid toml?")
    )]
    TomlParse(#[from] toml::de::Error),
    #[error("Failed to convert config to toml: {0}")]
    #[diagnostic(code(mit_lint::model::lints::error::toml_serialize), url(docsrs))]
    TomlSerialize(#[from] toml::ser::Error),
}
