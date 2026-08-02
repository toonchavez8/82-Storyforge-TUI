use ratatui::layout::Rect;

/// Minimum terminal width for the UI to render anything useful.
const MIN_WIDTH: u16 = 80;
/// Minimum terminal height for the UI to render anything useful.
const MIN_HEIGHT: u16 = 24;
/// Width at which the UI can switch from compact to standard layout.
const STANDARD_WIDTH: u16 = 100;
/// Height at which the UI can switch from compact to standard layout.
const STANDARD_HEIGHT: u16 = 30;

/// Layout modes chosen from the live terminal size.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LayoutMode {
    /// Terminal is too small to render the game UI.
    TooSmall,
    /// Reduced layout for smaller terminals.
    Compact,
    /// Full layout for large terminals.
    Standard,
}

/// Selects the right layout mode for the given terminal area.
#[must_use]
pub const fn mode_for(area: Rect) -> LayoutMode {
    if area.width < MIN_WIDTH || area.height < MIN_HEIGHT {
        LayoutMode::TooSmall
    } else if area.width < STANDARD_WIDTH || area.height < STANDARD_HEIGHT {
        LayoutMode::Compact
    } else {
        LayoutMode::Standard
    }
}

#[cfg(test)]
mod tests {
    use ratatui::layout::Rect;

    use super::{LayoutMode, mode_for};

    #[test]
    fn mode_should_be_too_small_below_minimum_width() {
        assert_eq!(mode_for(Rect::new(0, 0, 79, 24)), LayoutMode::TooSmall);
    }

    #[test]
    fn mode_should_be_compact_at_minimum_size() {
        assert_eq!(mode_for(Rect::new(0, 0, 80, 24)), LayoutMode::Compact);
    }

    #[test]
    fn mode_should_be_standard_at_recommended_size() {
        assert_eq!(mode_for(Rect::new(0, 0, 120, 36)), LayoutMode::Standard);
    }
}
