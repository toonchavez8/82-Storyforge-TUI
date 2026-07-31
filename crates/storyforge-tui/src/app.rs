use std::io;

use crossterm::event::{self, Event, KeyCode, KeyEventKind};

use ratatui::{
    DefaultTerminal, Frame,
    layout::Alignment,
    widgets::{Block, Borders, Paragraph},
};

// keep the app state in one struct so the rest of the program can ask
// whether the UI should keep running or shut down.
#[derive(Debug, Default)]
pub struct App {
    //flip this to true when the user presses a quit key.
    should_quit: bool,
}

impl App {
    // take ownership of the app state here and keep drawing frames until
    // something tells the app to quit.
    pub fn run(mut self, terminal: &mut DefaultTerminal) -> io::Result<()> {
        // I keep looping while the app is still active.
        while !self.should_quit {
            // ask ratatui to redraw the full screen using my render function.
            terminal.draw(Self::render)?;
            // wait for the next terminal event, then let the app decide what
            // to do with it.
            self.handle_event(&event::read()?);
        }

        // return Ok once the user has chosen to leave the app.
        Ok(())
    }

    // render the current UI frame. This does not need self yet because the
    // screen is static for now.
    fn render(frame: &mut Frame) {
        // build the text widget that appears in the terminal.
        let message = Paragraph::new("Storyforge is awake! \n \n Press q or Esc to leave.")
            .alignment(Alignment::Center)
            .block(Block::default().title("Storyforge").borders(Borders::ALL));

        // draw the widget into the full available terminal area.
        frame.render_widget(message, frame.area());
    }

    // inspect input events and update my app state when one matters.
    fn handle_event(&mut self, event: &Event) {
        // we only care about keyboard events here, so mouse/resize/focus events
        // are ignored.
        let Event::Key(key) = event else {
            return;
        };

        // only react to the initial key press, not repeat or release events.
        if key.kind != KeyEventKind::Press {
            return;
        }

        // mark the app as done when the user presses q or Esc.
        if matches!(key.code, KeyCode::Char('q') | KeyCode::Esc) {
            self.should_quit = true;
        }
    }
}

#[cfg(test)]
mod tests {
    use crossterm::event::{Event, KeyCode, KeyEvent, KeyModifiers};

    use super::App;

    #[test]
    fn q_should_request_quit() {
        // start with a fresh app that should still be running.
        let mut app = App::default();
        // create the same kind of key event the terminal would send for q.
        let key = KeyEvent::new(KeyCode::Char('q'), KeyModifiers::NONE);

        // send the key event into the app and expect it to request shutdown.
        app.handle_event(&Event::Key(key));
        assert!(app.should_quit);
    }
}
