use std::collections::{BTreeMap, BTreeSet};

use crate::{Diagnostic, LoadedCampaign, Severity, schema_version};

/// Validates a fully loaded campaign and returns every issue that was found.
///
/// Validation intentionally collects all problems instead of stopping at the
/// first one. This gives campaign authors a complete report so they can fix
/// multiple issues in a single iteration.
#[must_use]
pub fn validate_campaign(campaign: &LoadedCampaign) -> Vec<Diagnostic> {
    let mut diagnostics = Vec::new();

    // Ensure the campaign was authored against a schema version
    // that this engine understands.
    if campaign.manifest.schema_version != schema_version() {
        diagnostics.push(Diagnostic {
            severity: Severity::Error,
            code: "manifest.unsupported-schema",
            message: format!(
                "schema version {} is not supported",
                campaign.manifest.schema_version
            ),
            path: Some(campaign.root.join("campaign.toml")),
        });
    }

    // Count how many times each scene ID appears so duplicate
    // identifiers can be reported later.
    let mut scene_counts: BTreeMap<&str, usize> = BTreeMap::new();
    for scene in &campaign.scenes {
        *scene_counts.entry(scene.id.as_str()).or_default() += 1;
    }

    // Every scene ID should be unique.
    for (id, count) in &scene_counts {
        if *count > 1 {
            diagnostics.push(Diagnostic {
                severity: Severity::Error,
                code: "scene.duplicate-id",
                message: format!("scene ID `{id}` appears {count} times"),
                path: None,
            });
        }
    }

    // Build a lookup set to make existence checks inexpensive.
    let scene_ids = scene_counts.keys().copied().collect::<BTreeSet<_>>();

    // The manifest's entry scene must point to a real scene.
    if !scene_ids.contains(campaign.manifest.entry_scene.as_str()) {
        diagnostics.push(Diagnostic {
            severity: Severity::Error,
            code: "manifest.missing-entry-scene",
            message: format!(
                "entry scene `{}` does not exist",
                campaign.manifest.entry_scene
            ),
            path: Some(campaign.root.join("campaign.toml")),
        });
    }

    // Validate every scene independently.
    for scene in &campaign.scenes {
        // Choice IDs only need to be unique within their own scene.
        let mut choice_ids = BTreeSet::new();

        for choice in &scene.choices {
            // Detect duplicate choice IDs.
            if !choice_ids.insert(choice.id.as_str()) {
                diagnostics.push(Diagnostic {
                    severity: Severity::Error,
                    code: "scene.duplicate-choice-id",
                    message: format!("scene `{}` repeats choice ID `{}`", scene.id, choice.id),
                    path: None,
                });
            }

            // Every choice in a non-terminal scene should lead somewhere.
            if !scene.terminal && choice.target.is_none() {
                diagnostics.push(Diagnostic {
                    severity: Severity::Error,
                    code: "scene.choice-missing-target",
                    message: format!(
                        "choice `{}` in scene `{}` has no target",
                        choice.id, scene.id
                    ),
                    path: None,
                });
            }

            // If a target is specified, make sure the destination exists.
            if let Some(target) = &choice.target {
                if !scene_ids.contains(target.as_str()) {
                    diagnostics.push(Diagnostic {
                        severity: Severity::Error,
                        code: "scene.missing-target",
                        message: format!(
                            "choice `{}` in scene `{}` targets missing scene `{}`",
                            choice.id, scene.id, target
                        ),
                        path: None,
                    });
                }
            }
        }

        // Non-terminal scenes should always provide at least one
        // path forward for the player.
        if !scene.terminal && scene.choices.is_empty() {
            diagnostics.push(Diagnostic {
                severity: Severity::Error,
                code: "scene.no-choices",
                message: format!("nonterminal scene `{}` has no choices", scene.id),
                path: None,
            });
        }
    }

    // Keep the output deterministic so validation results are
    // stable across runs and easier to compare in tests or CI.
    diagnostics.sort_by(|left, right| {
        left.path
            .cmp(&right.path)
            .then(left.code.cmp(right.code))
            .then(left.message.cmp(&right.message))
    });

    diagnostics
}
