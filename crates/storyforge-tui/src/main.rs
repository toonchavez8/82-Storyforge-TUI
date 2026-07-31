//! Terminal executable for Storyforge.

mod action;
mod app;
mod layout;
mod theme;
mod ui;

use color_eyre::Result;

use app::App;

fn main() -> Result<()> {
    color_eyre::install()?;
    ratatui::run(|terminal| App::default().run(terminal))?;
    Ok(())
}
