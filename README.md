# Storyforge TUI

Storyforge TUI is a Rust terminal RPG engine for story-driven campaigns with deterministic rules, validated content packs, visible dice rolls, tactical choices, saves, and replayable consequences.

Current milestone: M0 boot workspace. The Rust workspace exists, the crates compile, and the executable prints the engine/schema diagnostic.

## Run locally

```powershell
cargo run -p storyforge-tui
```

Expected output:

```text
Storyforge content schema 1
```

## Workspace layout

```text
storyforge-tui/
├── campaigns/
│   └── academy-demo/
├── crates/
│   ├── storyforge-core/
│   ├── storyforge-content/
│   └── storyforge-tui/
├── Cargo.toml
├── README.md
└── rust-toolchain.toml
```

## Crate boundaries

- `storyforge-core`: deterministic game rules and state transitions. It must not depend on terminal rendering or filesystem behavior.
- `storyforge-content`: campaign loading and validation. It depends on `storyforge-core`.
- `storyforge-tui`: executable boundary for terminal UI, command-line behavior, saves, logs, and user-facing errors.

## Content policy

- `campaigns/academy-demo` is the public original demo campaign.
- Private or licensed reference material must stay out of this repository.
- A private campaign pack such as `wizarding-world-private` should live as a sibling private Git repository, not inside this public workspace.
- Local private paths can be stored in ignored config such as `.storyforge.local.toml`.

## Simplified roadmap

Use these task lists as the live project checklist.

### M0: boot workspace

- [x] Create Rust workspace with three crates.
- [x] Add pinned Rust toolchain.
- [x] Add workspace lints.
- [x] Add generated/private file ignores.
- [x] Print engine and content schema diagnostic.
- [ ] Commit the initial workspace.

### M1: terminal shell and character creation

- [ ] Add terminal lifecycle setup and cleanup.
- [ ] Add responsive dashboard shell.
- [ ] Add input handling and app state loop.
- [ ] Add player character data model.
- [ ] Add character creation flow.
- [ ] Validate character choices before confirmation.

### M2: first playable MVP

- [ ] Load `academy-demo` content from disk.
- [ ] Validate pack manifest and stable content IDs.
- [ ] Add one branching scene.
- [ ] Add visible d20 skill checks.
- [ ] Add one short tactical duel.
- [ ] Add one consequence or reward.
- [ ] Add save, quit, relaunch, and continue.

### M3: explorable alpha

- [ ] Add location graph and travel.
- [ ] Add inventory and shops.
- [ ] Add school schedule, classes, and time.
- [ ] Add quests and objective tracking.
- [ ] Add NPC schedules.
- [ ] Add cantrips, spell slots, flexible casting, and metamagic.
- [ ] Add content validation command.

### M4: campaign beta

- [ ] Add factions and reputation.
- [ ] Add companions and relationship memory.
- [ ] Add world phases and regional consequences.
- [ ] Add deeper enemy AI.
- [ ] Add arc gates.
- [ ] Add ending evaluation from accumulated state.

### M5: public release

- [ ] Add `storyforge validate --pack <path>`.
- [ ] Add `storyforge doctor`.
- [ ] Add release archives.
- [ ] Add npm/npx launcher packaging.
- [ ] Add checksum validation for bundled native binaries.
- [ ] Verify private content is absent from release artifacts.

### Later version candidates

- [ ] Living school schedules, restocking, and rumors.
- [ ] Relationship gifts, companion downtime, and loyalty missions.
- [ ] Faction-controlled regions and regional consequences.
- [ ] Reactions, terrain, nonlethal objectives, and expanded combat reports.
- [ ] Investigation, correspondence, crafting, spatial travel, and continuity systems.

## Three-arc campaign support

The engine should support a full three-arc campaign without hard-coding one plot.

- Arc I, Reconstruction, levels 1-10: school life, local mysteries, relationships, factions, and first irreversible decisions.
- Arc II, Fracture, levels 11-16: travel, unstable portals, institutional conflict, alliances, betrayals, and continuity from Arc I.
- Arc III, Convergence, levels 17-20: endgame locations, multi-stage plans, boss encounters, artifacts, alliances, and accumulated-state endings.

The detailed plot belongs in campaign data. The engine should model arcs, conditions, effects, world variables, and ending rules.

## Verification

Run these before milestone commits:

```powershell
cargo fmt --all
cargo check --workspace --locked
cargo clippy --workspace --all-targets --all-features --locked -- -D warnings
cargo test --workspace --all-features --locked
cargo run -p storyforge-tui
git status --short
```

## License

This workspace currently declares `Apache-2.0` in `Cargo.toml`.
