use crossterm::event::{self, Event};
use ratatui::DefaultTerminal;
use std::io;

use crate::{action::UiAction, theme::Theme, ui};

/// Holds every piece of visible application state.
///
/// `render()` only reads from `App`, so tests and snapshots can construct a
/// state, draw it, and compare the output without touching the terminal.
#[derive(Debug)]
pub struct App {
    /// Set to `true` when the user chooses to leave the app.
    pub(crate) should_quit: bool,
    /// The screen currently shown to the player.
    pub(crate) screen: Screen,
    /// Which compact-mode tab or standard-mode pane has keyboard focus.
    pub(crate) focus: Focus,
    /// Index of the selected compact tab, wrapped with modulo so it stays valid.
    pub(crate) selected_tab: usize,
    /// Active theme used by every widget. Stored here so the renderer and
    /// future settings commands share the same palette.
    pub(crate) theme: Theme,
    /// Current spell-slot count for levels 1 through 9.
    pub(crate) spell_slots_current: [u8; 9],
    /// Maximum spell-slot count for levels 1 through 9.
    pub(crate) spell_slots_max: [u8; 9],
    /// Temporary spell-slot count for levels 1 through 9.
    pub(crate) spell_slots_temp: [u8; 9],
    /// `(current, max)` sorcery points. `None` means the character does not
    /// have the feature, so the UI hides the SP row entirely.
    pub(crate) sorcery_points: Option<(u8, u8)>,
}

/// Top-level screens the player can open.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum Screen {
    #[default]
    Story,
    Character,
    Journal,
    Map,
    Help,
}

/// Keyboard focus target inside a screen.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum Focus {
    #[default]
    Actions,
    /// Reserved for a future story-pane focus state.
    #[expect(dead_code)]
    Story,
    Log,
}

impl Default for App {
    fn default() -> Self {
        Self {
            should_quit: false,
            screen: Screen::default(),
            focus: Focus::default(),
            selected_tab: 0,
            theme: Theme::default(),
            // Demo caster: 1st-level slots plus sorcery points so the layout
            // has realistic spell-resource data to render.
            spell_slots_current: [3, 2, 0, 0, 0, 0, 0, 0, 0],
            spell_slots_max: [4, 2, 0, 0, 0, 0, 0, 0, 0],
            spell_slots_temp: [0; 9],
            sorcery_points: Some((3, 3)),
        }
    }
}

impl App {
    /// Owns the application state and runs the main loop until the user quits.
    /// Rendering is delegated to `ui::render`, so this loop stays focused on
    /// events and state updates.
    ///
    /// # Errors
    ///
    /// Returns an I/O error if ratatui cannot draw a frame or read input.
    pub fn run(mut self, terminal: &mut DefaultTerminal) -> io::Result<()> {
        while !self.should_quit {
            terminal.draw(|frame| ui::render(frame, &self))?;
            self.handle_event(&event::read()?);
        }

        Ok(())
    }

    /// Handles one crossterm event.
    ///
    /// Mouse, resize, and focus events are ignored here. Resize still reaches
    /// `render` through the live frame area, so the layout adapts automatically.
    fn handle_event(&mut self, event: &Event) {
        let Event::Key(key) = event else {
            return;
        };

        // Convert the raw key into a semantic UI action before applying it.
        // This keeps the event loop small and makes keyboard mappings easy to
        // test in isolation.
        let action = UiAction::from(*key);
        self.update(action);
    }

    /// Applies one UI action to the application state.
    ///
    /// All navigation and quitting flows through here, which keeps the behavior
    /// centralized and deterministic.
    fn update(&mut self, action: UiAction) {
        match action {
            UiAction::OpenCharacter => self.screen = Screen::Character,
            UiAction::OpenJournal => self.screen = Screen::Journal,
            UiAction::OpenMap => self.screen = Screen::Map,
            UiAction::OpenHelp => self.screen = Screen::Help,
            // Back from a subscreen returns to the story screen; from the story
            // screen it quits, matching the Esc key behavior.
            UiAction::Back if self.screen != Screen::Story => self.screen = Screen::Story,
            UiAction::Back | UiAction::Quit => self.should_quit = true,
            // Up, Down, and Confirm are reserved for future lists and forms.
            UiAction::Up | UiAction::Down | UiAction::Confirm | UiAction::None => {}
        }
    }
}

#[cfg(test)]
mod tests {
    use crossterm::event::{Event, KeyCode, KeyEvent, KeyModifiers};

    use super::App;

    #[test]
    fn q_should_request_quit() {
        let mut app = App::default();
        let key = KeyEvent::new(KeyCode::Char('q'), KeyModifiers::NONE);

        app.handle_event(&Event::Key(key));

        assert!(app.should_quit);
    }
}
