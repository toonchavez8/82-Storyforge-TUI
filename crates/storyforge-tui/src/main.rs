//! Terminal executable for Storyforge.
mod app;

use color_eyre::Result;

use app::App;

fn main() -> Result<()> {
    color_eyre::install()?;
    ratatui::run(|terminal| App::default().run(terminal))?;
    Ok(())
}
