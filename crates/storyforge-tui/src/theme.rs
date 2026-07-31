use ratatui::style::Color;

/// Semantic color palette for every widget in the TUI.
///
/// Widgets ask for named colors like `accent` or `danger` instead of inventing
/// their own RGB values. Centralizing the palette makes theming and snapshot
/// testing straightforward.
#[derive(Debug, Clone, Copy)]
pub struct Theme {
    pub background: Color,
    pub text: Color,
    pub muted: Color,
    pub accent: Color,
    pub success: Color,
    pub danger: Color,
    pub focus: Color,
}

impl Default for Theme {
    fn default() -> Self {
        Self {
            background: Color::Rgb(10, 10, 21),
            text: Color::Rgb(51, 152, 219),
            muted: Color::Rgb(112, 127, 245),
            accent: Color::Rgb(231, 126, 35),
            success: Color::Rgb(26, 188, 156),
            danger: Color::Rgb(231, 37, 35),
            focus: Color::Rgb(241, 196, 14),
        }
    }
}
