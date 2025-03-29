use core::cmp::min;
use crossterm::event::{
    Event::{self, Key},
    KeyCode, KeyEvent, KeyEventKind, KeyModifiers, read,
};

mod terminal;
use terminal::{Position, Size, Terminal};

mod view;
use view::View;

#[derive(Copy, Clone, Default)]
struct Location {
    x: usize,
    y: usize,
}

pub struct Editor {
    should_quit: bool,
    location: Location,
    view: View,
}


impl Editor {
    pub fn default() -> Self {
        Self {
            should_quit: false,
            location: Location::default(),
            view: View::default(),
        }
    }

    pub fn run(&mut self) {
        Terminal::initialize().unwrap();
        let result = self.repl();
        Terminal::terminate().unwrap();
        result.unwrap();
    }

    fn repl(&mut self) -> Result<(), std::io::Error> {
        loop {
            self.refresh_screen()?;
            if self.should_quit {
                break;
            }
            let event = read()?;
            self.evaluate_event(&event)?;
        }
        Ok(())
    }

    fn evaluate_event(&mut self, event: &Event) -> Result<(), std::io::Error> {
        if let Key(KeyEvent {
            code,
            modifiers,
            kind: KeyEventKind::Press,
            ..
        }) = event
        {
            match code {
                KeyCode::Char('q') if *modifiers == KeyModifiers::CONTROL => {
                    self.should_quit = true;
                }
                KeyCode::Left
                | KeyCode::Right
                | KeyCode::Up
                | KeyCode::Down
                | KeyCode::Home
                | KeyCode::End
                | KeyCode::PageUp
                | KeyCode::PageDown => {
                    let size = Terminal::size()?;
                    self.location = Self::move_point(*code, self.location, size);
                }
                _ => (),
            }
        }
        Ok(())
    }

    fn refresh_screen(&self) -> Result<(), std::io::Error> {
        Terminal::hide_cursor()?;
        if self.should_quit {
            Terminal::clear_screen()?;
            Terminal::move_cursor_to(Position::default())?;
            Terminal::print("Goodbye!\r\n")?;
        } else {
            Terminal::move_cursor_to(Position::default())?;
            self.view.render()?;
            Terminal::move_cursor_to(Position {
                col: self.location.x,
                row: self.location.y,
            })?;
        }
        Terminal::show_cursor()?;
        Terminal::execute()?;
        Ok(())
    }

    fn move_point(
        key_code: KeyCode,
        location: Location,
        size: Size,
    ) -> Location {
        let Location { mut x, mut y } = location;
        let Size { height, width } = size;
        match key_code {
            KeyCode::Left => {
                x = x.saturating_sub(1);
            }
            KeyCode::Right => {
                x = min(x.saturating_add(1), width.saturating_sub(1));
            }
            KeyCode::Up => {
                y = y.saturating_sub(1);
            }
            KeyCode::Down => {
                y = min(y.saturating_add(1), height.saturating_sub(1));
            }
            KeyCode::Home => {
                x = 0;
            }
            KeyCode::End => {
                x = width.saturating_sub(1);
            }
            KeyCode::PageUp => {
                y = 0;
            }
            KeyCode::PageDown => {
                y = height.saturating_sub(1);
            }
            _ => (),
        }
        Location { x, y }
    }

}
