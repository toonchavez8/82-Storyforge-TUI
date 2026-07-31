use crossterm::event::{KeyCode, KeyEvent, KeyEventKind};

/// A semantic action the user can ask the UI to perform.
///
/// Converting raw key events into these actions keeps the event loop small and
/// makes the update logic easy to read and test.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum UiAction {
    Up,
    Down,
    Confirm,
    Back,
    OpenCharacter,
    OpenJournal,
    OpenMap,
    OpenHelp,
    Quit,
    None,
}

/// Converts a crossterm key event into the UI action it represents.
///
/// Release and repeat events map to `None` so the app only acts once per
/// physical key press.
impl From<KeyEvent> for UiAction {
    fn from(key: KeyEvent) -> Self {
        if key.kind != KeyEventKind::Press {
            return Self::None;
        }

        match key.code {
            KeyCode::Up => Self::Up,
            KeyCode::Down => Self::Down,
            KeyCode::Enter => Self::Confirm,
            KeyCode::Esc => Self::Back,
            KeyCode::Char('c') => Self::OpenCharacter,
            KeyCode::Char('j') => Self::OpenJournal,
            KeyCode::Char('m') => Self::OpenMap,
            KeyCode::Char('?') => Self::OpenHelp,
            KeyCode::Char('q') => Self::Quit,
            _ => Self::None,
        }
    }
}
