use rand::{RngCore, SeedableRng};
use rand_chacha::ChaCha8Rng;

use crate::{GameCommand, GameEvent, GameState};

/// Owns the authoritative game state and processes player commands.
///
/// The engine follows a simple command → event → state flow:
///
/// 1. Receive a `GameCommand`.
/// 2. Produce one or more `GameEvent`s.
/// 3. Apply those events to the game state.
/// 4. Record the events in the event log.
///
#[derive(Debug)]
pub struct GameEngine {
    /// Current state of the game.
    state: GameState,

    /// Random number generator used for deterministic gameplay.
    ///
    /// Because the RNG is seeded, the same sequence of commands with the same
    /// seed will always produce the same results.
    rng: ChaCha8Rng,
}

impl GameEngine {
    /// Creates a new game engine.
    ///
    /// # Parameters
    ///
    /// * `seed` - Seed used to initialize the random number generator.
    ///
    /// # Returns
    ///
    /// A new engine with a default game state and a deterministic RNG.
    #[must_use]
    pub fn new(state: GameState, seed: u64) -> Self {
        Self {
            state,
            rng: ChaCha8Rng::seed_from_u64(seed),
        }
    }

    /// Returns an immutable reference to the current game state.
    ///
    /// Callers can inspect the state without being able to modify it directly.
    #[must_use]
    pub fn state(&self) -> &GameState {
        &self.state
    }

    /// Executes a player command.
    ///
    /// The command itself never modifies the game state directly. Instead it is
    /// converted into one or more events, those events are applied to the
    /// state, and finally they are appended to the event log.
    ///
    /// # Parameters
    ///
    /// * `command` - The player action to execute.
    ///
    /// # Returns
    ///
    /// The list of events produced while handling the command.
    #[allow(clippy::needless_pass_by_value)]
    pub fn dispatch(&mut self, command: GameCommand) -> Vec<GameEvent> {
        // Determine what should happen.
        let events = handle_command(&self.state, &command);

        // Apply the resulting events to the game state.
        apply_events(&mut self.state, &events);

        // Preserve a history of everything that happened.
        self.state.event_log.extend(events.iter().cloned());

        events
    }

    /// Returns the next deterministic random `u32`.
    ///
    /// The sequence is entirely determined by the seed supplied when the
    /// `GameEngine` was created, making gameplay reproducible in tests.
    #[must_use]
    pub fn next_random_u32(&mut self) -> u32 {
        self.rng.next_u32()
    }
}

/// Converts a command into a list of game events.
///
/// This function contains the game's rules. It examines the current state
/// and decides which events should occur, but it does **not** mutate the
/// state itself.
///
/// Keeping event generation separate from event application makes the logic
/// easier to test and replay.
#[must_use]
pub fn handle_command(state: &GameState, command: &GameCommand) -> Vec<GameEvent> {
    let choice_count = match command {
        GameCommand::SelectNextChoice { choice_count }
        | GameCommand::SelectPreviousChoice { choice_count } => *choice_count,
    };

    if choice_count == 0 {
        return vec![GameEvent::CommandRejected {
            reason: "the active scene has no choices".to_owned(),
        }];
    }

    let previous = state.selected_choice;

    let current = match command {
        GameCommand::SelectNextChoice { .. } => (previous + 1) % choice_count,
        GameCommand::SelectPreviousChoice { .. } => {
            previous.checked_sub(1).unwrap_or(choice_count - 1)
        }
    };
    let next_turn = state.turn.saturating_add(1);
    vec![
        GameEvent::ChoiceSelectionChanged { previous, current },
        GameEvent::TurnAdvanced { turn: next_turn },
    ]
}

/// Applies previously generated events to the game state.
///
/// This function is intentionally "dumb." It does not make gameplay decisions;
/// it simply updates the state to reflect the supplied events.
///
/// # Parameters
///
/// * `state` - Game state to update.
/// * `events` - Events to apply in chronological order.
pub fn apply_events(state: &mut GameState, events: &[GameEvent]) {
    // Events must be applied in the order they were generated.
    for event in events {
        match event {
            // Update the selected choice.
            GameEvent::ChoiceSelectionChanged { current, .. } => {
                state.selected_choice = *current;
            }

            // Advance the game's turn counter.
            GameEvent::TurnAdvanced { turn } => {
                state.turn = *turn;
            }

            // Rejected commands don't currently modify state.
            // Future implementations could increment statistics or write
            // additional diagnostics here.
            GameEvent::CommandRejected { .. } => {}
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{ContentId, GameCommand, GameEngine, GameEvent, GameState};

    fn engine() -> Result<GameEngine, crate::IdError> {
        let scene = ContentId::new("academy.scene.arrival")?;
        Ok(GameEngine::new(GameState::new(scene), 42))
    }

    #[test]
    fn next_choice_should_advance_selection() -> Result<(), crate::IdError> {
        let mut engine = engine()?;

        let events = engine.dispatch(GameCommand::SelectNextChoice { choice_count: 3 });

        assert!(matches!(
            events.as_slice(),
            [
                GameEvent::ChoiceSelectionChanged {
                    previous: 0,
                    current: 1
                },
                GameEvent::TurnAdvanced { turn: 1 }
            ]
        ));
        assert_eq!(engine.state().selected_choice, 1);
        Ok(())
    }

    #[test]
    fn previous_choice_should_wrap_at_start() -> Result<(), crate::IdError> {
        let mut engine = engine()?;

        engine.dispatch(GameCommand::SelectPreviousChoice { choice_count: 3 });

        assert_eq!(engine.state().selected_choice, 2);
        Ok(())
    }

    #[test]
    fn empty_choice_list_should_emit_rejection() -> Result<(), crate::IdError> {
        let mut engine = engine()?;

        let events = engine.dispatch(GameCommand::SelectNextChoice { choice_count: 0 });

        assert!(matches!(
            events.as_slice(),
            [GameEvent::CommandRejected { reason }]
                if reason == "the active scene has no choices"
        ));
        assert_eq!(engine.state().selected_choice, 0);
        Ok(())
    }
}
