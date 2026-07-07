//! Serde support for `regex::Regex`
//!
//! `Regex` does not implement `Serialize` or `Deserialize` because the
//! pattern string is not sufficient to reconstruct a compiled regex on its
//! own. Lint configuration structs embed `Regex` fields, and we want those
//! structs to round-trip through a config file (TOML, JSON, ...).
//!
//! This module provides a transparent helper that serialises a `Regex` as
//! its source string and deserialises it back via `Regex::new`.
//!
//! Use it with `#[serde(with = "crate::model::serde_regex")]` on any
//! `Regex` field.

use regex::Regex;
use serde::{Deserialize, Deserializer, Serialize, Serializer};

/// Serialise a `Regex` as its pattern string.
///
/// # Errors
///
/// Never returns an error -- `Regex::as_str` is infallible.
pub fn serialize<S>(regex: &Regex, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    regex.as_str().serialize(serializer)
}

/// Deserialise a `Regex` from a pattern string.
///
/// # Errors
///
/// Returns a deserialisation error if the string is not a valid regex.
pub fn deserialize<'de, D>(deserializer: D) -> Result<Regex, D::Error>
where
    D: Deserializer<'de>,
{
    let pattern = String::deserialize(deserializer)?;
    Regex::new(&pattern).map_err(serde::de::Error::custom)
}

/// A wrapper that implements `Serialize`/`Deserialize` for `Regex`.
///
/// Use this when you need a standalone serialisable regex type, e.g. as a
/// struct field without the `#[serde(with = ...)]` attribute.
#[derive(Debug, Clone)]
pub struct SerializableRegex(pub Regex);

impl Serialize for SerializableRegex {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.0.as_str().serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for SerializableRegex {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let pattern = String::deserialize(deserializer)?;
        Regex::new(&pattern)
            .map(SerializableRegex)
            .map_err(serde::de::Error::custom)
    }
}

impl From<Regex> for SerializableRegex {
    fn from(regex: Regex) -> Self {
        Self(regex)
    }
}

impl From<&Regex> for SerializableRegex {
    fn from(regex: &Regex) -> Self {
        Self(regex.clone())
    }
}

impl std::fmt::Display for SerializableRegex {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0.as_str())
    }
}

impl std::ops::Deref for SerializableRegex {
    type Target = Regex;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl PartialEq for SerializableRegex {
    fn eq(&self, other: &Self) -> bool {
        self.0.as_str() == other.0.as_str()
    }
}

impl Eq for SerializableRegex {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn round_trip_regex_through_serde() {
        let regex = Regex::new(r"^foo\d+$").unwrap();
        let json = serde_json::to_string(&SerializableRegex(regex)).unwrap();
        let deserialised: SerializableRegex = serde_json::from_str(&json).unwrap();
        assert_eq!(deserialised.0.as_str(), r"^foo\d+$");
    }

    #[test]
    fn deserialise_invalid_regex_fails() {
        let json = r#""[invalid(""#;
        let result: Result<SerializableRegex, _> = serde_json::from_str(json);
        assert!(result.is_err());
    }

    #[test]
    fn serializable_regex_display() {
        let regex = SerializableRegex(Regex::new(r"\d+").unwrap());
        assert_eq!(format!("{regex}"), r"\d+");
    }

    #[test]
    fn serializable_regex_deref() {
        let regex = SerializableRegex(Regex::new(r"abc").unwrap());
        assert!(regex.is_match("abc"));
    }
}

#[cfg(kani)]
mod proofs {
    use std::collections::hash_map::DefaultHasher;
    use std::hash::{Hash, Hasher};

    use super::*;

    #[kani::proof]
    fn serde_roundtrip_preserves_pattern() {
        // Concrete proof: serialise then deserialise a known regex and
        // verify the pattern string survives the round trip.
        let original = SerializableRegex::from(Regex::new(r"\d+").unwrap());
        let json = serde_json::to_string(&original).unwrap();
        let recovered: SerializableRegex = serde_json::from_str(&json).unwrap();
        assert_eq!(
            original, recovered,
            "serde round-trip must preserve the regex"
        );
    }

    #[kani::proof]
    fn equal_regexes_hash_equally() {
        // Hash/Eq contract: if two SerializableRegex values are equal,
        // their hashes must be equal too. We use two independently
        // constructed regexes with the same pattern.
        let a = SerializableRegex::from(Regex::new(r"^foo\d+$").unwrap());
        let b = SerializableRegex::from(Regex::new(r"^foo\d+$").unwrap());

        assert_eq!(a, b, "same pattern must be equal");

        let mut ha = DefaultHasher::new();
        a.hash(&mut ha);
        let mut hb = DefaultHasher::new();
        b.hash(&mut hb);
        assert_eq!(ha.finish(), hb.finish(), "equal values must hash equally");
    }

    #[kani::proof]
    fn display_matches_pattern() {
        let regex = SerializableRegex::from(Regex::new(r"abc").unwrap());
        assert_eq!(format!("{regex}"), r"abc");
    }
}
