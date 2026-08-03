use std::{fmt, str::FromStr};

use serde::{Deserialize, Deserializer, Serialize, de::Error as _};

/// A validated identifier used to reference game content.
///
/// Internally this is just a `String`, but construction is restricted so every
/// instance is guaranteed to follow the expected format.
///
/// Examples of valid IDs:
/// - `story.intro`
/// - `npc.shopkeeper`
/// - `item.health_potion`
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize)]
#[serde(transparent)]
pub struct ContentId(String);

impl ContentId {
    /// Creates a validated content ID.
    ///
    /// # Parameters
    ///
    /// * `value` - Any type that can be converted into a `String`
    ///   (typically `&str` or `String`).
    ///
    /// # Returns
    ///
    /// - `Ok(ContentId)` if the identifier is valid.
    /// - `Err(IdError)` if the identifier is malformed.
    ///
    /// A valid identifier must:
    /// - not be empty
    /// - contain at least one namespace separator (`.`)
    /// - not contain empty namespace segments (`item..potion`)
    /// - only use lowercase ASCII letters, digits, `.`, `_`, and `-`
    /// # Errors
    ///
    /// Returns [`IdError::Invalid`] if:
    /// - the value is empty
    /// - it does not contain a namespace separator (`.`)
    /// - it contains empty namespace segments (for example `item..potion`)
    /// - it contains characters other than lowercase ASCII letters, digits,
    ///   `.`, `_`, or `-`
    pub fn new(value: impl Into<String>) -> Result<Self, IdError> {
        // Accept either &str or String without forcing callers
        // to perform an allocation beforehand.
        let value = value.into();

        // A content ID is considered namespaced when it contains at least one
        // '.' and none of the segments are empty.
        //
        // Valid:
        //     item.potion
        //     npc.blacksmith.shop
        //
        // Invalid:
        //     potion
        //     .potion
        //     item.
        //     item..potion
        let has_namespace = value.contains('.') && !value.split('.').any(str::is_empty);

        // Ensure every character belongs to our supported character set.
        //
        // This intentionally rejects:
        // - uppercase letters
        // - whitespace
        // - unicode
        // - punctuation other than '.', '_' and '-'
        let valid_characters = value.chars().all(|character| {
            character.is_ascii_lowercase()
                || character.is_ascii_digit()
                || matches!(character, '.' | '_' | '-')
        });

        // Reject anything that fails one of the validation rules.
        if value.is_empty() || !has_namespace || !valid_characters {
            return Err(IdError::Invalid(value));
        }

        // At this point the identifier is guaranteed to satisfy every rule.
        Ok(Self(value))
    }

    /// Returns the underlying identifier as a borrowed string slice.
    ///
    /// No allocation is performed.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

/// Allows `ContentId` to be printed with `{}`.
///
/// Since the internal representation is already a `String`, we simply forward
/// formatting to that string.
impl fmt::Display for ContentId {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(formatter)
    }
}

/// Allows `"item.sword".parse::<ContentId>()`.
///
/// This delegates to `ContentId::new()` so all validation rules remain in one
/// place.
impl FromStr for ContentId {
    type Err = IdError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::new(s)
    }
}

/// Custom deserialization so invalid IDs are rejected while loading JSON,
/// YAML, TOML, or any other Serde-supported format.
///
/// Instead of accepting any string, we deserialize the raw string first and
/// then run the exact same validation used everywhere else.
impl<'de> Deserialize<'de> for ContentId {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        // Read the raw string from the serialized data.
        let value = String::deserialize(deserializer)?;

        // Validate it before constructing a ContentId.
        Self::new(value).map_err(D::Error::custom)
    }
}

/// Errors that can occur while constructing a `ContentId`.
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum IdError {
    /// The supplied identifier failed one or more validation rules.
    #[error("content ID `{0}` must be lowercase, namespaced, and ASCII")]
    Invalid(String),
}
