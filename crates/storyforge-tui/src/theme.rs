use ratatui::style::Color;

/// Identifies one of the built-in house themes.
///
/// The names are deliberately neutral versions of the house animals so the
/// public engine does not ship copyrighted names. Private campaign packs can
/// display whatever names they want in their own content.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ThemeId {
    /// Deep red and gold.
    Lion,

    /// Dark blue and bronze.
    Raven,

    /// Black and yellow.
    Badger,

    /// Dark green and silver.
    Serpent,
}

impl ThemeId {
    /// Returns the display name used in the UI header.
    #[must_use]
    pub fn name(self) -> &'static str {
        match self {
            Self::Lion => "Lion",
            Self::Raven => "Raven",
            Self::Badger => "Badger",
            Self::Serpent => "Serpent",
        }
    }

    /// Returns the next theme in the cycle, wrapping back to the start.
    #[must_use]
    pub fn next(self) -> Self {
        match self {
            Self::Lion => Self::Raven,
            Self::Raven => Self::Badger,
            Self::Badger => Self::Serpent,
            Self::Serpent => Self::Lion,
        }
    }
}

/// Semantic color palette for every widget in the TUI.
///
/// Widgets ask for named colors like `accent` or `danger` instead of inventing
/// their own RGB values. Centralizing the palette makes theming and snapshot
/// testing straightforward.
///
/// `primary` and `secondary` are reserved for future buttons and highlighted
/// controls. They are not used anywhere yet, but they live here so no other
/// file invents a one-off color when that time comes.
#[derive(Debug, Clone, Copy)]
pub struct Theme {
    pub background: Color,
    pub text: Color,
    pub muted: Color,
    pub accent: Color,
    pub success: Color,
    pub danger: Color,
    pub focus: Color,
    pub primary: Color,
    pub secondary: Color,
}

impl Theme {
    /// Returns the theme that matches the given `ThemeId`.
    #[must_use]
    pub fn for_id(id: ThemeId) -> Self {
        match id {
            ThemeId::Lion => Self::lion(),
            ThemeId::Raven => Self::raven(),
            ThemeId::Badger => Self::badger(),
            ThemeId::Serpent => Self::serpent(),
        }
    }

    /// Lion theme: deep red and gold.
    #[must_use]
    pub fn lion() -> Self {
        Self {
            background: Color::Rgb(12, 0, 0),
            text: Color::Rgb(248, 226, 209),
            muted: Color::Rgb(135, 0, 1),
            accent: Color::Rgb(217, 124, 33),
            success: Color::Rgb(26, 188, 156),
            danger: Color::Rgb(231, 37, 35),
            focus: Color::Rgb(255, 186, 51),
            primary: Color::Rgb(211, 166, 37),
            secondary: Color::Rgb(135, 0, 1),
        }
    }

    /// Raven theme: dark blue and bronze.
    #[must_use]
    pub fn raven() -> Self {
        Self {
            background: Color::Rgb(9, 24, 48),
            text: Color::Rgb(230, 240, 255),
            muted: Color::Rgb(60, 80, 120),
            accent: Color::Rgb(135, 135, 135),
            success: Color::Rgb(26, 188, 156),
            danger: Color::Rgb(231, 37, 35),
            focus: Color::Rgb(70, 130, 240),
            primary: Color::Rgb(30, 60, 130),
            secondary: Color::Rgb(184, 115, 51),
        }
    }

    /// Badger theme: black and yellow.
    #[must_use]
    pub fn badger() -> Self {
        Self {
            background: Color::Rgb(10, 3, 4),
            text: Color::Rgb(255, 232, 180),
            muted: Color::Rgb(55, 46, 41),
            accent: Color::Rgb(229, 179, 55),
            success: Color::Rgb(26, 188, 156),
            danger: Color::Rgb(231, 37, 35),
            focus: Color::Rgb(255, 224, 0),
            primary: Color::Rgb(255, 215, 0),
            secondary: Color::Rgb(20, 15, 5),
        }
    }

    /// Serpent theme: dark green and silver.
    #[must_use]
    pub fn serpent() -> Self {
        Self {
            background: Color::Rgb(5, 20, 10),
            text: Color::Rgb(220, 245, 235),
            muted: Color::Rgb(40, 90, 60),
            accent: Color::Rgb(192, 192, 192),
            success: Color::Rgb(26, 188, 156),
            danger: Color::Rgb(231, 37, 35),
            focus: Color::Rgb(80, 220, 120),
            primary: Color::Rgb(30, 120, 60),
            secondary: Color::Rgb(192, 192, 192),
        }
    }

    /// Returns a copy of this theme with every RGB color multiplied by 3/4.
    ///
    /// This dims unfocused panels by 25%. Non-RGB colors are left unchanged.
    /// The fraction is easy to change later if you want a subtler 15% or 20%
    /// effect.
    #[must_use]
    pub fn dim(self) -> Self {
        const NUMERATOR: u16 = 3;
        const DENOMINATOR: u16 = 4;

        Self {
            background: dim_color(self.background, NUMERATOR, DENOMINATOR),
            text: dim_color(self.text, NUMERATOR, DENOMINATOR),
            muted: dim_color(self.muted, NUMERATOR, DENOMINATOR),
            accent: dim_color(self.accent, NUMERATOR, DENOMINATOR),
            success: dim_color(self.success, NUMERATOR, DENOMINATOR),
            danger: dim_color(self.danger, NUMERATOR, DENOMINATOR),
            focus: dim_color(self.focus, NUMERATOR, DENOMINATOR),
            primary: dim_color(self.primary, NUMERATOR, DENOMINATOR),
            secondary: dim_color(self.secondary, NUMERATOR, DENOMINATOR),
        }
    }
}

impl Default for Theme {
    /// A neutral academy theme used for tests and snapshots.
    ///
    /// The play path starts with the Lion theme, but keeping a neutral default
    /// means snapshot tests do not need to know which house is currently active.
    fn default() -> Self {
        Self {
            background: Color::Rgb(10, 10, 21),
            text: Color::Rgb(51, 152, 219),
            muted: Color::Rgb(112, 127, 245),
            accent: Color::Rgb(231, 126, 35),
            success: Color::Rgb(26, 188, 156),
            danger: Color::Rgb(231, 37, 35),
            focus: Color::Rgb(241, 196, 14),
            primary: Color::Rgb(231, 126, 35),
            secondary: Color::Rgb(112, 127, 245),
        }
    }
}

/// Dims a single RGB color by an integer fraction.
///
/// Uses `u16` for the intermediate multiplication so channel values up to
/// 255 do not overflow.
fn dim_color(color: Color, numerator: u16, denominator: u16) -> Color {
    match color {
        Color::Rgb(r, g, b) => Color::Rgb(
            ((r as u16 * numerator) / denominator) as u8,
            ((g as u16 * numerator) / denominator) as u8,
            ((b as u16 * numerator) / denominator) as u8,
        ),
        other => other,
    }
}

#[cfg(test)]
mod tests {
    use super::{Color, Theme, ThemeId, dim_color};

    #[test]
    fn dim_color_should_darken_rgb_by_the_given_fraction() {
        let color = Color::Rgb(100, 120, 140);
        let dimmed = dim_color(color, 3, 4);

        assert_eq!(dimmed, Color::Rgb(75, 90, 105));
    }

    #[test]
    fn dim_color_should_leave_non_rgb_colors_unchanged() {
        assert_eq!(dim_color(Color::Reset, 3, 4), Color::Reset);
    }

    #[test]
    fn theme_dim_should_darken_every_channel() {
        let bright = Theme::lion();
        let dimmed = bright.dim();

        // The background is the easiest channel to reason about.
        assert_eq!(dimmed.background, Color::Rgb(9, 0, 0));
    }

    #[test]
    fn theme_id_should_cycle_through_houses() {
        assert_eq!(ThemeId::Lion.next(), ThemeId::Raven);
        assert_eq!(ThemeId::Raven.next(), ThemeId::Badger);
        assert_eq!(ThemeId::Badger.next(), ThemeId::Serpent);
        assert_eq!(ThemeId::Serpent.next(), ThemeId::Lion);
    }

    #[test]
    fn theme_id_name_should_match_the_house() {
        assert_eq!(ThemeId::Lion.name(), "Lion");
        assert_eq!(ThemeId::Raven.name(), "Raven");
        assert_eq!(ThemeId::Badger.name(), "Badger");
        assert_eq!(ThemeId::Serpent.name(), "Serpent");
    }
}
