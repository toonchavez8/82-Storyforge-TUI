//! Terminal executable for Storyforge.

fn main() {
    println!(
        "{} content schema {}",
        storyforge_core::engine_name(),
        storyforge_content::schema_version()
    );
}
