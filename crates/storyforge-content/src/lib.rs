//! Campaign loading and validation for Storyforge.

mod error;
mod loader;
mod model;
mod validation;

pub use error::{ContentError, Diagnostic, Severity};
pub use loader::{LoadedCampaign, load_campaign};
pub use model::{CampaignManifest, ChoiceDefinition, SceneDefinition};
pub use validation::validate_campaign;

/// Returns the current content schema version.
#[must_use]
pub const fn schema_version() -> u32 {
    1
}
