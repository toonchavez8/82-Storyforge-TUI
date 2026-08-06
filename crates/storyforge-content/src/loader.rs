use std::{
    fs,
    path::{Path, PathBuf},
};

use crate::{CampaignManifest, ContentError, SceneDefinition};

/// Represents a campaign after it has been loaded into memory.
///
/// Keeping the manifest and all parsed scenes together makes it easy
/// for the game engine to navigate the story without repeatedly
/// reading files from disk.
#[derive(Debug)]
pub struct LoadedCampaign {
    /// Root folder where the campaign was loaded from.
    pub root: PathBuf,

    /// Campaign metadata parsed from `campaign.toml`.
    pub manifest: CampaignManifest,

    /// Every scene discovered inside the campaign's `scenes` directory.
    pub scenes: Vec<SceneDefinition>,
}

impl LoadedCampaign {
    /// Returns a scene matching the given content ID.
    ///
    /// This is the main lookup method used by the runtime whenever
    /// it needs to move the player to another scene.
    #[must_use]
    pub fn scene(&self, id: &storyforge_core::ContentId) -> Option<&SceneDefinition> {
        self.scenes.iter().find(|s| &s.id == id)
    }
}

/// Loads an entire campaign from disk.
///
/// The process is intentionally straightforward:
/// 1. Read the campaign manifest.
/// 2. Discover every `.ron` scene file.
/// 3. Sort the files to guarantee deterministic loading.
/// 4. Parse each scene into memory.
///
/// Sorting the file list avoids subtle differences between
/// operating systems where directory iteration order is not guaranteed.
///
/// # Errors
///
/// Returns [`ContentError`] if any required file cannot be read
/// or if the manifest/scene data contains invalid syntax.
pub fn load_campaign(root: &Path) -> Result<LoadedCampaign, ContentError> {
    // Every campaign starts with its manifest.
    let manifest_path = root.join("campaign.toml");

    let manifest_text =
        fs::read_to_string(&manifest_path).map_err(|source| ContentError::Read {
            path: manifest_path.clone(),
            source,
        })?;

    // Parse the TOML manifest into a strongly typed structure.
    let manifest = toml::from_str(&manifest_text).map_err(|source| ContentError::Manifest {
        path: manifest_path,
        source,
    })?;

    let scene_dir = root.join("scenes");

    // Discover every scene file inside the campaign.
    // Nested folders are supported so campaigns can be organized
    // however their authors prefer.
    let mut paths = walkdir::WalkDir::new(&scene_dir)
        .min_depth(1)
        .max_depth(8)
        .into_iter()
        .filter_map(Result::ok)
        .filter(|entry| entry.file_type().is_file())
        .map(walkdir::DirEntry::into_path)
        .filter(|path| path.extension().is_some_and(|extension| extension == "ron"))
        .collect::<Vec<_>>();

    // Keep loading deterministic regardless of filesystem ordering.
    paths.sort();

    // Parse every discovered scene.
    // If one scene fails, the entire load is aborted so the campaign
    // never starts in a partially loaded state.
    let scenes = paths
        .iter()
        .map(|path| load_scene(path))
        .collect::<Result<Vec<_>, _>>()?;

    Ok(LoadedCampaign {
        root: root.to_path_buf(),
        manifest,
        scenes,
    })
}

/// Reads and parses a single scene file.
///
/// Keeping scene loading isolated makes it easier to reuse this logic
/// later for validation tools or editors.
///
/// # Errors
///
/// Returns [`ContentError::Read`] when the file cannot be opened,
/// or [`ContentError::Scene`] when the RON data cannot be parsed.
fn load_scene(path: &Path) -> Result<SceneDefinition, ContentError> {
    // Load the raw scene text from disk.
    let text = fs::read_to_string(path).map_err(|source| ContentError::Read {
        path: path.to_path_buf(),
        source,
    })?;

    // Convert the RON document into its Rust representation.
    ron::from_str(&text).map_err(|source| ContentError::Scene {
        path: path.to_path_buf(),
        source: Box::new(source),
    })
}

#[cfg(test)]
mod tests {
    use std::path::Path;

    use super::load_campaign;

    /// Basic integration test that verifies the demo campaign
    /// bundled with the repository can be loaded successfully.
    ///
    /// If this test fails, it usually means a manifest or scene
    /// file was edited incorrectly.
    #[test]
    fn academy_demo_campaign_should_load() {
        let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("../../campaigns/academy-demo");

        let campaign = load_campaign(&root).expect("academy demo campaign should load");

        // The entry scene declared in the manifest should always exist.
        assert!(campaign.scene(&campaign.manifest.entry_scene).is_some());
    }
}
