//! Deterministic rules and state transitions for Storyforge campaigns.

/// Returns the engine name used in diagnostics.
#[must_use]
pub const fn engine_name() -> &'static str {
    "Storyforge"
}

#[cfg(test)]
mod tests {
    use super::engine_name;

    #[test]
    fn engine_name_should_be_stable() {
        assert_eq!(engine_name(), "Storyforge");
    }
}
