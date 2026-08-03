use serde::{Deserialize, Serialize};

use crate::ContentId;

/// Represents the current state of an active game session.
///
/// This structure contains all information needed to resume gameplay or save
/// progress. As more gameplay systems are added, additional player state,
/// inventory, quests, and flags can be stored here.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct GameState {
    /// Identifier of the scene currently being shown to the player.
    pub active_scene: ContentId,

    /// Index of the currently highlighted choice within the active scene.
    ///
    /// The UI updates this as the player navigates the available options.
    pub selected_choice: usize,

    /// Number of turns that have elapsed since the game started.
    ///
    /// This can be used for gameplay logic, scripting, or analytics.
    pub turn: u64,

    /// Chronological record of gameplay events that have occurred.
    ///
    /// This is useful for debugging, replay systems, or displaying a history
    /// to the player.
    pub event_log: Vec<GameEvent>,
}

impl GameState {
    /// Creates a new game state starting at the given scene.
    ///
    /// # Parameters
    ///
    /// * `active_scene` - The first scene that should become active.
    ///
    /// # Returns
    ///
    /// A fresh `GameState` where:
    /// - the supplied scene becomes the active scene
    /// - the first choice is selected (`0`)
    /// - the turn counter starts at `0`
    /// - the event log is empty
    #[must_use]
    pub fn new(active_scene: ContentId) -> Self {
        Self {
            active_scene,

            // Start by highlighting the first available choice.
            // Should always be valid index into the scene's choice list.
            selected_choice: 0,

            // No turns have elapsed yet.
            // should never decrease
            turn: 0,

            // No gameplay events have occurred yet.
            // Should be ordered chronologically.
            event_log: Vec::new(),
        }
    }
}

/// Represents an action or notable occurrence during gameplay.
///
/// Events provide a history of what happened and can be used for debugging,
/// replay functionality, achievements, or analytics.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum GameEvent {
    /// The player moved the cursor to a different choice.
    ///
    /// Stores the previously selected choice index.
    ChoiceSelectionChanged {
        /// Choice index before the selection changed.
        previous: usize,
        /// Choice index after the selection changed.
        current: usize,
    },

    /// The player attempted an action that could not be completed.
    ///
    /// The reason is intended for logging or developer diagnostics.
    CommandRejected {
        /// Human-readable explanation of why the command failed.
        reason: String,
    },

    /// The game's turn counter advanced.
    TurnAdvanced {
        /// The new current turn number.
        turn: u64,
    },
}
