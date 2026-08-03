/// A player action that requests the game state to change.
///
/// Commands represent *intent*, not the result of that intent. The game state
/// is responsible for validating a command and applying any resulting changes.
///
/// For example, attempting to move to the next choice does not guarantee the
/// selection will change if there are no available choices.
pub enum GameCommand {
    /// Requests that the selection move to the next available choice.
    ///
    /// # Fields
    ///
    /// * `choice_count` - Total number of choices currently available in the
    ///   active scene. This allows the game state to wrap or clamp the
    ///   selection without needing to query the scene itself.
    SelectNextChoice {
        /// Number of available choices in the active scene.
        choice_count: usize,
    },

    /// Requests that the selection move to the previous available choice.
    ///
    /// # Fields
    ///
    /// * `choice_count` - Total number of choices currently available in the
    ///   active scene.
    SelectPreviousChoice {
        /// Number of available choices in the active scene.
        choice_count: usize,
    },
}
