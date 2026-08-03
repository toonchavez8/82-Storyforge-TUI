use crossterm::event::{KeyCode, KeyEvent, KeyEventKind};

/// A semantic action the user can ask the UI to perform.
///
/// Converting raw key events into these actions keeps the event loop small and
/// makes the update logic easy to read and test.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum UiAction {
    /// Move up inside the currently focused panel.
    MoveUp,

    /// Move down inside the currently focused panel.
    MoveDown,

    /// Move left inside the currently focused panel, or fall back to the
    /// previous panel if the panel has no horizontal layout.
    MoveLeft,

    /// Move right inside the currently focused panel, or fall back to the next
    /// panel if the panel has no horizontal layout.
    MoveRight,

    /// Move focus to the next panel in the dashboard layout.
    FocusNext,

    /// Move focus to the previous panel in the dashboard layout.
    FocusPrevious,

    /// Open or close the inventory sidebar.
    ToggleInventory,

    /// Switch to the next color theme.
    NextTheme,

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
            //Arrow keys and WASD keys for movement inside the currently focused panel.
            KeyCode::Up | KeyCode::Char('w') => Self::MoveUp,
            KeyCode::Down | KeyCode::Char('s') => Self::MoveDown,
            KeyCode::Left | KeyCode::Char('a') => Self::MoveLeft,
            KeyCode::Right | KeyCode::Char('d') => Self::MoveRight,

            //j/k movement move focus to the next/previous panel in the dashboard layout.
            KeyCode::Char('j') => Self::FocusNext,
            KeyCode::Char('k') => Self::FocusPrevious,

            // i key to toggle inventory sidebar
            KeyCode::Char('i') => Self::ToggleInventory,

            // t key to cycle to the next color theme
            KeyCode::Char('t') => Self::NextTheme,

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
    use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

    use super::UiAction;

    #[test]
    fn arrow_keys_should_map_to_directional_movement() {
        let up = KeyEvent::new(KeyCode::Up, KeyModifiers::NONE);
        let down = KeyEvent::new(KeyCode::Down, KeyModifiers::NONE);
        let left = KeyEvent::new(KeyCode::Left, KeyModifiers::NONE);
        let right = KeyEvent::new(KeyCode::Right, KeyModifiers::NONE);

        assert_eq!(UiAction::from(up), UiAction::MoveUp);
        assert_eq!(UiAction::from(down), UiAction::MoveDown);
        assert_eq!(UiAction::from(left), UiAction::MoveLeft);
        assert_eq!(UiAction::from(right), UiAction::MoveRight);
    }

    #[test]
    fn wasd_keys_should_map_to_directional_movement() {
        let up = KeyEvent::new(KeyCode::Char('w'), KeyModifiers::NONE);
        let down = KeyEvent::new(KeyCode::Char('s'), KeyModifiers::NONE);
        let left = KeyEvent::new(KeyCode::Char('a'), KeyModifiers::NONE);
        let right = KeyEvent::new(KeyCode::Char('d'), KeyModifiers::NONE);

        assert_eq!(UiAction::from(up), UiAction::MoveUp);
        assert_eq!(UiAction::from(down), UiAction::MoveDown);
        assert_eq!(UiAction::from(left), UiAction::MoveLeft);
        assert_eq!(UiAction::from(right), UiAction::MoveRight);
    }

    #[test]
    fn j_and_k_should_move_focus() {
        let next = KeyEvent::new(KeyCode::Char('j'), KeyModifiers::NONE);
        let previous = KeyEvent::new(KeyCode::Char('k'), KeyModifiers::NONE);

        assert_eq!(UiAction::from(next), UiAction::FocusNext);
        assert_eq!(UiAction::from(previous), UiAction::FocusPrevious);
    }

    #[test]
    fn i_should_toggle_inventory() {
        let key = KeyEvent::new(KeyCode::Char('i'), KeyModifiers::NONE);

        assert_eq!(UiAction::from(key), UiAction::ToggleInventory);
    }

    #[test]
    fn t_should_cycle_to_the_next_theme() {
        let key = KeyEvent::new(KeyCode::Char('t'), KeyModifiers::NONE);

        assert_eq!(UiAction::from(key), UiAction::NextTheme);
    }

    #[test]
    fn l_should_open_journal() {
        let key = KeyEvent::new(KeyCode::Char('l'), KeyModifiers::NONE);

        assert_eq!(UiAction::from(key), UiAction::OpenJournal);
    }
}
