use crossterm::event::{self, Event};
use ratatui::DefaultTerminal;
use std::io;

use crate::{action::UiAction, theme::Theme, ui};
use storyforge_core::{ContentId, GameCommand, GameEngine, GameState};

/// Holds every piece of visible application state.
///
/// The `App` is responsible for application-level concerns such as navigation,
/// keyboard focus, UI state, and owning the gameplay engine.
///
/// Rendering only reads from this structure, making it easy to construct an
/// `App` during tests and render snapshots without creating a real terminal.
#[derive(Debug)]
pub struct App {
    /// Set to `true` once the user has requested to exit the application.
    pub(crate) should_quit: bool,

    /// The currently visible top-level screen.
    pub(crate) screen: Screen,

    /// Which pane or section currently owns keyboard focus.
    pub(crate) focus: Focus,

    /// Index of the currently selected compact-mode tab.
    ///
    /// The value is wrapped using modulo arithmetic so it always remains within
    /// the valid tab range.
    pub(crate) selected_tab: usize,

    /// Deterministic gameplay engine.
    ///
    /// The TUI never implements game rules directly. Instead it converts user
    /// input into commands and sends them to the engine, which owns the game's
    /// authoritative state.
    pub(crate) engine: GameEngine,

    /// Active color theme used throughout the UI.
    ///
    /// Keeping the theme in the application state allows future settings
    /// screens to modify it without changing the renderer.
    pub(crate) theme: Theme,

    /// Current spell-slot count for spell levels 1 through 9.
    pub(crate) spell_slots_current: [u8; 9],

    /// Maximum spell-slot count for spell levels 1 through 9.
    pub(crate) spell_slots_max: [u8; 9],

    /// Temporary spell-slot count for spell levels 1 through 9.
    pub(crate) spell_slots_temp: [u8; 9],

    /// Current and maximum sorcery points.
    ///
    /// A value of `None` indicates the character does not have the Sorcery
    /// Points feature, so the UI hides that row completely.
    pub(crate) sorcery_points: Option<(u8, u8)>,
}

/// Top-level screens the player can navigate between.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum Screen {
    /// Interactive story view.
    #[default]
    Story,

    /// Character sheet.
    Character,

    /// Quest and event journal.
    Journal,

    /// World map.
    Map,

    /// Help and keyboard shortcuts.
    Help,
}

/// Indicates which area of the interface currently receives keyboard input.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum Focus {
    /// Action list or choice list.
    #[default]
    Actions,

    /// Reserved for future story text navigation.
    #[expect(dead_code)]
    Story,

    /// Event or combat log.
    Log,
}

impl Default for App {
    fn default() -> Self {
        // Temporary bootstrap scene.
        //
        // Later guides will load this from the campaign manifest instead of
        // hard-coding it here.
        let active_scene = ContentId::new("academy.scene.arrival")
            .expect("temporary built-in scene ID should be valid");

        Self {
            should_quit: false,
            screen: Screen::default(),
            focus: Focus::default(),
            selected_tab: 0,

            // Create the deterministic gameplay engine.
            //
            // Seed `42` is used temporarily during development so gameplay is
            // completely reproducible. Future save files will provide the seed.
            engine: GameEngine::new(GameState::new(active_scene), 42),

            theme: Theme::default(),

            // Demo spell resources so the spell panel has meaningful data while
            // the real character system is still under development.
            spell_slots_current: [3, 2, 0, 0, 0, 0, 0, 0, 0],
            spell_slots_max: [4, 2, 0, 0, 0, 0, 0, 0, 0],
            spell_slots_temp: [0; 9],

            // Demo sorcery points for layout testing.
            sorcery_points: Some((3, 3)),
        }
    }
}

impl App {
    /// Runs the application's main loop.
    ///
    /// The loop repeatedly:
    ///
    /// 1. Renders the current UI.
    /// 2. Waits for the next terminal event.
    /// 3. Updates the application state.
    ///
    /// The loop exits once `should_quit` becomes `true`.
    ///
    /// # Errors
    ///
    /// Returns an I/O error if Ratatui cannot draw a frame or if Crossterm
    /// fails while reading terminal input.
    pub fn run(mut self, terminal: &mut DefaultTerminal) -> io::Result<()> {
        while !self.should_quit {
            // Draw the entire UI using the current application state.
            terminal.draw(|frame| ui::render(frame, &self))?;

            // Wait for the next terminal event.
            self.handle_event(&event::read()?);
        }

        Ok(())
    }

    /// Handles a single terminal event.
    ///
    /// Only keyboard events are currently processed. Mouse events, window
    /// resize notifications, and focus events are ignored.
    ///
    /// Window resizing still works because Ratatui provides the current frame
    /// dimensions every time `render()` is called.
    fn handle_event(&mut self, event: &Event) {
        let Event::Key(key) = event else {
            return;
        };

        // Convert the raw keyboard input into a semantic application action.
        //
        // This keeps keyboard bindings isolated from application logic, making
        // both easier to test independently.
        let action = UiAction::from(*key);

        self.update(action);
    }

    /// Applies a semantic UI action to the application state.
    ///
    /// All application navigation flows through this method, giving the project
    /// a single location responsible for mutating UI state.
    fn update(&mut self, action: UiAction) {
        match action {
            UiAction::OpenCharacter => self.screen = Screen::Character,

            UiAction::OpenJournal => self.screen = Screen::Journal,

            UiAction::OpenMap => self.screen = Screen::Map,

            UiAction::OpenHelp => self.screen = Screen::Help,

            // Esc returns from any secondary screen back to the story.
            // Pressing Esc again while already on the story exits the app.
            UiAction::Back if self.screen != Screen::Story => {
                self.screen = Screen::Story;
            }

            UiAction::Back | UiAction::Quit => {
                self.should_quit = true;
            }

            UiAction::Down if self.screen == Screen::Story => {
                self.engine.dispatch(GameCommand::SelectNextChoice {
                    choice_count: self.visible_choice_count(),
                });
            }
            UiAction::Up if self.screen == Screen::Story => {
                self.engine.dispatch(GameCommand::SelectPreviousChoice {
                    choice_count: self.visible_choice_count(),
                });
            }
            UiAction::Down => {
                self.selected_tab = (self.selected_tab + 1) % 4;
            }
            UiAction::Up => {
                self.selected_tab = self.selected_tab.checked_sub(1).unwrap_or(3);
            }

            // Confirmation behavior will eventually dispatch gameplay commands
            // to the engine.
            UiAction::Confirm | UiAction::None => {}
        }
    }

    /// Returns the number of visible choices for the current tab.
    fn visible_choice_count(&self) -> usize {
        2
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

    #[test]
    fn movement_should_cycle_through_tabs() {
        let mut app = App::default();

        // Put the app on any screen where Up/Down controls tabs.
        app.screen = crate::app::Screen::Character;

        app.update(crate::action::UiAction::Down);
        assert_eq!(app.selected_tab, 1);

        app.update(crate::action::UiAction::Down);
        assert_eq!(app.selected_tab, 2);

        app.update(crate::action::UiAction::Down);
        assert_eq!(app.selected_tab, 3);

        app.update(crate::action::UiAction::Down);
        assert_eq!(app.selected_tab, 0);

        app.update(crate::action::UiAction::Up);
        assert_eq!(app.selected_tab, 3);

        app.update(crate::action::UiAction::Up);
        assert_eq!(app.selected_tab, 2);

        app.update(crate::action::UiAction::Up);
        assert_eq!(app.selected_tab, 1);

        app.update(crate::action::UiAction::Up);
        assert_eq!(app.selected_tab, 0);
    }

    #[test]
    fn down_on_story_should_dispatch_to_the_engine() {
        let mut app = App::default();

        app.update(crate::action::UiAction::Down);

        assert_eq!(app.engine.state().selected_choice, 1);
    }
}
