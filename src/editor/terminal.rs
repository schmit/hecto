use crossterm::terminal::{disable_raw_mode, enable_raw_mode, Clear, ClearType, size};
use crossterm::execute;
use crossterm::cursor::MoveTo;

pub struct Terminal {}


impl Terminal {
    pub fn size() -> Result<(u16, u16), std::io::Error> {
        size()
    }

    pub fn initialize() -> Result<(), std::io::Error> {
        enable_raw_mode()?;
        Self::clear_screen()?;
        Ok(())
    }

    pub fn terminate() -> Result<(), std::io::Error> {
        disable_raw_mode()
    }

    pub fn clear_screen() -> Result<(), std::io::Error> {
        let mut stdout = std::io::stdout();
        execute!(stdout, Clear(ClearType::All))
    }

    pub fn move_cursor_to(x: u16, y: u16) -> Result<(), std::io::Error> {
        let mut stdout = std::io::stdout();
        execute!(stdout, MoveTo(x, y))
    }
}
