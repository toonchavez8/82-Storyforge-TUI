use serde::Deserialize;
use storyforge_core::ContentId;

/// Basic information that describes a campaign.
///
/// This is loaded once when the game starts and tells the engine
/// where the adventure begins along with a few metadata fields.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct CampaignManifest {
    /// Unique identifier for the campaign.
    pub id: String,

    /// Display name shown to the player.
    pub name: String,

    /// Campaign version (useful for updates or compatibility checks).
    pub version: String,

    /// Version of the manifest format expected by the engine.
    pub schema_version: u32,

    /// Ruleset or gameplay profile this campaign uses.
    pub rules_profile: String,

    /// Locale to use if the player has not selected one.
    pub default_locale: String,

    /// Scene where a new game begins.
    pub entry_scene: ContentId,

    /// Story arc that is active when the campaign starts.
    pub starting_arc: String,
}

/// Represents a single scene in the story.
///
/// A scene contains the narrative text, the choices available
/// to the player, and whether it marks the end of a path.
#[derive(Debug, Clone, Deserialize)]
pub struct SceneDefinition {
    /// Unique identifier for this scene.
    pub id: ContentId,

    /// Title displayed at the top of the scene.
    pub title: String,

    /// Narrative text split into paragraphs.
    pub body: Vec<String>,

    /// Choices the player can select from this scene.
    pub choices: Vec<ChoiceDefinition>,

    /// Indicates whether this scene has no further progression.
    pub terminal: bool,
}

/// A single option the player can choose.
///
/// Most choices lead to another scene, but some may intentionally
/// have no target (for example, actions handled by game logic).
#[derive(Debug, Clone, Deserialize)]
pub struct ChoiceDefinition {
    /// Internal identifier for the choice.
    pub id: String,

    /// Text shown to the player.
    pub label: String,

    /// Destination scene after selecting this choice.
    /// A value of `None` means the engine is expected to
    /// resolve the outcome through another mechanism.
    pub target: Option<ContentId>,
}
