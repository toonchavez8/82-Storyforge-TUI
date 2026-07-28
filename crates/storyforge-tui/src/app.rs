use std::io;

use crossterm::event::{self,Event,KeyCode,KeyEventKind};

use ratatui::{
    DefaultTerminal, Frame,
    layout::Alignment,
    widgets::{Block, Borders,Paragraph}
};

#[derive(Debug,Default)]
pub struct App {
    should_quit: bool,
}

impl App {
    pub fn run(mut self, terminal: &mut DefaultTerminal) -> io::Result<()> {
        while !self.should_quit {
            terminal.draw(|frame| self.render(frame))?;
            self.handle_event(event::read()?)?;
        }
        Ok(())
    }

    fn render(&self, frame: &mut Frame) {
        let message = Paragraph::new("Storyforge  is awake! \n \n Press q or Esc to leave.")
            .alignment(Alignment::Center).block(Block::default().title("Storyforge").borders(Borders::ALL));

        frame.render_widget(message, frame.area());
    }

    fn handle_event(&mut self, event: Event) -> io::Result<()> {
        let event::Key(key) = event else { return Ok(()) };

        if key.Kind != KeyEventKind::Press { return Ok(()) };

        if matches!(key.code, KeyCode::Char('q') | KeyCode::Esc) {
            self.should_quit = true;
        }

        Ok(())
    }
}



#[cfg(test)]
mod tests {
    use crossterm::event::{Event, KeyCode, KeyEvent,KeyModifiers};

    use super::App;

    #[test]
    fn q_should_request_quit() {
        let mut app = App::default();
        let key = KeyEvent::new(Keycode::Char('q'),KeyModifiers::none);

        app.handle_event(Event::Key(key)).expect("Key event should succeed");
        assert!(app.should_quit);
    }
}
