//! Core gameplay engine for Storyforge.
//!
//! This crate owns the game's state, command processing, event generation,
//! and deterministic simulation. User interfaces should interact with the
//! engine by dispatching `GameCommand`s and rendering the resulting
//! `GameState`.
mod command;
mod engine;
mod id;
mod state;

pub use command::GameCommand;
pub use engine::{GameEngine, apply_events, handle_command};
pub use id::{ContentId, IdError};
pub use state::{GameEvent, GameState};
