use crossterm::event::{self, Event};
use ratatui::DefaultTerminal;
use std::io;

use crate::{action::UiAction, theme::Theme, theme::ThemeId, ui};
use storyforge_core::{ContentId, GameCommand, GameEngine, GameState};

/// Number of columns used for the temporary inventory grid.
///
/// This is a UI constant, not gameplay state. It will eventually be computed
/// from the rendered inventory panel width.
const INVENTORY_COLUMNS: usize = 2;

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

    /// Which house palette is currently active.
    ///
    /// The full palette is recomputed from this id whenever it changes. The id
    /// is the small piece of state that is saved and compared; the RGB values
    /// live in `theme.rs`.
    pub(crate) theme_id: ThemeId,

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

    /// Whether the inventory sidebar is open.
    pub(crate) inventory_open: bool,

    /// Index of the currently selected item in the inventory grid.
    pub(crate) inventory_selected: usize,

    /// Placeholder inventory items for the sidebar.
    ///
    /// This will be replaced by the real inventory from `storyforge-core` once
    /// character and item systems exist.
    pub(crate) inventory_items: Vec<String>,
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
    /// Main narrative panel. Will later support scrolling prose.
    #[default]
    Story,

    /// Action list or choice list.
    Actions,

    /// Character summary panel.
    Character,

    /// Quest tracker panel.
    Quest,

    /// Event or combat log.
    Log,

    /// Inventory sidebar.
    Inventory,
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

            // Start in the Lion theme so the first screen has a distinct house
            // palette. Snapshots and tests that do not care about the house can
            // use `Theme::default()`.
            theme_id: ThemeId::Lion,
            theme: Theme::for_id(ThemeId::Lion),

            // Demo spell resources so the spell panel has meaningful data while
            // the real character system is still under development.
            spell_slots_current: [3, 2, 0, 0, 0, 0, 0, 0, 0],
            spell_slots_max: [4, 2, 0, 0, 0, 0, 0, 0, 0],
            spell_slots_temp: [0; 9],

            // Demo sorcery points for layout testing.
            sorcery_points: Some((3, 3)),

            // Inventory starts closed with a small dummy list so the sidebar
            // can be rendered and navigated before real items exist.
            inventory_open: false,
            inventory_selected: 0,
            inventory_items: vec![
                "Wand".to_owned(),
                "Spellbook".to_owned(),
                "Potion".to_owned(),
                "Robe".to_owned(),
                "Key".to_owned(),
                "Scroll".to_owned(),
            ],
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

    /// Moves the inventory selection up one row, clamping at the top.
    fn move_inventory_up(&mut self) {
        self.inventory_selected = self.inventory_selected.saturating_sub(INVENTORY_COLUMNS);
    }

    /// Moves the inventory selection down one row, clamping at the last item.
    fn move_inventory_down(&mut self) {
        let max_index = self.inventory_items.len().saturating_sub(1);
        self.inventory_selected = self
            .inventory_selected
            .saturating_add(INVENTORY_COLUMNS)
            .min(max_index);
    }

    /// Moves the inventory selection left one slot, clamping at the first item.
    fn move_inventory_left(&mut self) {
        self.inventory_selected = self.inventory_selected.saturating_sub(1);
    }

    /// Moves the inventory selection right one slot, clamping at the last item.
    fn move_inventory_right(&mut self) {
        let max_index = self.inventory_items.len().saturating_sub(1);
        self.inventory_selected = (self.inventory_selected + 1).min(max_index);
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

            UiAction::NextTheme => {
                self.theme_id = self.theme_id.next();
                self.theme = Theme::for_id(self.theme_id);
            }

            UiAction::Back if self.screen != Screen::Story => {
                self.screen = Screen::Story;
            }

            UiAction::Back | UiAction::Quit => {
                self.should_quit = true;
            }

            // Inventory grid movement has priority when the inventory panel is focused.
            UiAction::MoveUp if self.focus == Focus::Inventory => self.move_inventory_up(),
            UiAction::MoveDown if self.focus == Focus::Inventory => self.move_inventory_down(),
            UiAction::MoveLeft if self.focus == Focus::Inventory => self.move_inventory_left(),
            UiAction::MoveRight if self.focus == Focus::Inventory => self.move_inventory_right(),

            // When the Actions panel is focused on the Story screen, up/down
            // dispatch the engine choice-selection commands.
            UiAction::MoveUp if self.focus == Focus::Actions && self.screen == Screen::Story => {
                self.engine.dispatch(GameCommand::SelectPreviousChoice {
                    choice_count: self.visible_choice_count(),
                });
            }
            UiAction::MoveDown if self.focus == Focus::Actions && self.screen == Screen::Story => {
                self.engine.dispatch(GameCommand::SelectNextChoice {
                    choice_count: self.visible_choice_count(),
                });
            }

            // j/k move focus around the dashboard ring. Left/right fall back to
            // the same focus motion when the focused panel has no horizontal
            // layout, so these arms intentionally share a body.
            UiAction::FocusNext | UiAction::MoveRight => {
                let next = self.next_focus();
                self.select_focus(next);
            }
            UiAction::FocusPrevious | UiAction::MoveLeft => {
                let previous = self.previous_focus();
                self.select_focus(previous);
            }

            // i opens the inventory sidebar and focuses it. Pressing i again
            // closes it and returns focus to the Actions panel.
            UiAction::ToggleInventory => {
                self.inventory_open = !self.inventory_open;
                if self.inventory_open {
                    self.select_focus(Focus::Inventory);
                } else {
                    self.select_focus(Focus::Actions);
                }
            }

            // Any unhandled movement or confirm/none action does nothing for now.
            // Story scrolling, character sheet paging, and log scrolling will live
            // here later.
            UiAction::MoveUp | UiAction::MoveDown | UiAction::Confirm | UiAction::None => {}
        }
    }

    fn next_focus(&self) -> Focus {
        match self.focus {
            Focus::Story => Focus::Actions,
            Focus::Actions => Focus::Character,
            Focus::Character => Focus::Quest,
            Focus::Quest => Focus::Log,
            Focus::Log => {
                if self.inventory_open {
                    Focus::Inventory
                } else {
                    Focus::Story
                }
            }
            Focus::Inventory => Focus::Story,
        }
    }

    /// Returns the previous focusable panel in the dashboard ring.
    fn previous_focus(&self) -> Focus {
        match self.focus {
            Focus::Story => {
                if self.inventory_open {
                    Focus::Inventory
                } else {
                    Focus::Log
                }
            }
            Focus::Actions => Focus::Story,
            Focus::Character => Focus::Actions,
            Focus::Quest => Focus::Character,
            Focus::Log => Focus::Quest,
            Focus::Inventory => Focus::Log,
        }
    }

    /// Changes the active focus and keeps the compact tab in sync.
    ///
    /// Compact mode only shows one detail panel at a time. When the player
    /// moves focus to a detail panel, `selected_tab` should update so the
    /// visible panel matches the focused one.
    fn select_focus(&mut self, focus: Focus) {
        self.focus = focus;
        if let Some(tab) = detail_panel_index(focus) {
            self.selected_tab = tab;
        }
    }

    /// Returns the number of visible choices for the current tab.
    //
    // `self` is unused right now because the demo always shows two choices. It
    // will be needed once the choice list comes from the engine state, so the
    // signature is kept stable.
    #[expect(
        clippy::unused_self,
        reason = "placeholder until engine provides choice count"
    )]
    fn visible_choice_count(&self) -> usize {
        2
    }
}

/// Maps detail-panel focus values to the compact tab index.
///
/// Story and Inventory are not part of the compact tab strip, so they return
/// `None`.
fn detail_panel_index(focus: Focus) -> Option<usize> {
    match focus {
        Focus::Actions => Some(0),
        Focus::Character => Some(1),
        Focus::Quest => Some(2),
        Focus::Log => Some(3),
        _ => None,
    }
}
#[cfg(test)]
mod tests {
    use crossterm::event::{Event, KeyCode, KeyEvent, KeyModifiers};

    use super::{App, Focus};
    use crate::theme::{Theme, ThemeId};

    #[test]
    fn q_should_request_quit() {
        let mut app = App::default();

        let key = KeyEvent::new(KeyCode::Char('q'), KeyModifiers::NONE);

        app.handle_event(&Event::Key(key));

        assert!(app.should_quit);
    }

    #[test]
    fn down_on_story_should_dispatch_to_the_engine() {
        let mut app = App::default();
        app.select_focus(Focus::Actions);
        app.update(crate::action::UiAction::MoveDown);

        assert_eq!(app.engine.state().selected_choice, 1);
    }

    #[test]
    fn focus_next_should_cycle_through_panels_and_skip_closed_inventory() {
        let mut app = App::default();

        app.select_focus(Focus::Actions);
        assert_eq!(app.focus, Focus::Actions);
        assert_eq!(app.selected_tab, 0);

        app.select_focus(Focus::Character);
        assert_eq!(app.selected_tab, 1);

        app.select_focus(Focus::Quest);
        assert_eq!(app.selected_tab, 2);

        app.select_focus(Focus::Log);
        assert_eq!(app.selected_tab, 3);

        // Inventory is closed, so next from Log wraps back to Story.
        app.select_focus(app.next_focus());
        assert_eq!(app.focus, Focus::Story);
        // Story is not a detail panel, so selected_tab stays where it was.
        assert_eq!(app.selected_tab, 3);
    }

    #[test]
    fn focus_previous_should_include_inventory_when_open() {
        let mut app = App {
            inventory_open: true,
            ..App::default()
        };

        app.select_focus(Focus::Story);

        app.select_focus(app.previous_focus());
        assert_eq!(app.focus, Focus::Inventory);
    }

    #[test]
    fn inventory_open_should_appear_in_the_focus_ring() {
        let mut app = App {
            inventory_open: true,
            focus: Focus::Log,
            ..App::default()
        };

        app.select_focus(app.next_focus());
        assert_eq!(app.focus, Focus::Inventory);

        app.select_focus(app.next_focus());
        assert_eq!(app.focus, Focus::Story);
    }

    #[test]
    fn next_theme_should_cycle_theme_id_and_refresh_theme() {
        let mut app = App::default();

        assert_eq!(app.theme_id, ThemeId::Lion);
        assert_eq!(app.theme.background, Theme::lion().background);

        app.update(crate::action::UiAction::NextTheme);
        assert_eq!(app.theme_id, ThemeId::Raven);
        assert_eq!(app.theme.background, Theme::raven().background);

        app.update(crate::action::UiAction::NextTheme);
        assert_eq!(app.theme_id, ThemeId::Badger);
        assert_eq!(app.theme.background, Theme::badger().background);

        app.update(crate::action::UiAction::NextTheme);
        assert_eq!(app.theme_id, ThemeId::Serpent);
        assert_eq!(app.theme.background, Theme::serpent().background);

        app.update(crate::action::UiAction::NextTheme);
        assert_eq!(app.theme_id, ThemeId::Lion);
        assert_eq!(app.theme.background, Theme::lion().background);
    }
}
