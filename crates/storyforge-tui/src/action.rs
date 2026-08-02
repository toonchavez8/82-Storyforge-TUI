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
            KeyCode::Up | KeyCode::Char('w' | 'k') => Self::Up,
            KeyCode::Down | KeyCode::Char('s' | 'j') => Self::Down,
            KeyCode::Enter => Self::Confirm,
            KeyCode::Esc => Self::Back,
            KeyCode::Char('c') => Self::OpenCharacter,
            KeyCode::Char('l') => Self::OpenJournal,
            KeyCode::Char('m') => Self::OpenMap,
            KeyCode::Char('?') => Self::OpenHelp,
            KeyCode::Char('q') => Self::Quit,
            _ => Self::None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::UiAction;
    use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

    #[test]
    fn vim_keys_map_to_correct_actions() {
        let down = KeyEvent::new(KeyCode::Char('j'), KeyModifiers::NONE);
        let up = KeyEvent::new(KeyCode::Char('k'), KeyModifiers::NONE);

        assert_eq!(UiAction::from(down), UiAction::Down);
        assert_eq!(UiAction::from(up), UiAction::Up);
    }

    #[test]
    fn wasd_keys_map_to_correct_actions() {
        let down = KeyEvent::new(KeyCode::Char('s'), KeyModifiers::NONE);
        let up = KeyEvent::new(KeyCode::Char('w'), KeyModifiers::NONE);

        assert_eq!(UiAction::from(down), UiAction::Down);
        assert_eq!(UiAction::from(up), UiAction::Up);
    }

    #[test]
    fn l_should_open_journel() {
        let l = KeyEvent::new(KeyCode::Char('l'), KeyModifiers::NONE);
        assert_eq!(UiAction::from(l), UiAction::OpenJournal);
    }
}
