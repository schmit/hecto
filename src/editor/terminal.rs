use std::io::{stdout, Write};
use crossterm::terminal::{disable_raw_mode, enable_raw_mode, Clear, ClearType, size};
use crossterm::queue;
use crossterm::cursor::MoveTo;


#[derive(Copy, Clone)]
pub struct Size {
    pub width: u16,
    pub height: u16
}

#[derive(Copy, Clone)]
pub struct Position {
    pub x: u16,
    pub y: u16
}

impl Position {
    pub fn default() -> Self {
        Self { x: 0, y: 0 }
    }
}

pub struct Terminal {}

impl Terminal {
    pub fn size() -> Result<Size, std::io::Error> {
        let (width, height) = size()?;
        Ok(Size { width, height })
    }

    pub fn initialize() -> Result<(), std::io::Error> {
        enable_raw_mode()?;
        Self::clear_screen()?;
        Self::move_cursor_to(Position::default())?;
        Self::execute()?;
        Ok(())
    }

    pub fn terminate() -> Result<(), std::io::Error> {
        disable_raw_mode()
    }

    pub fn clear_screen() -> Result<(), std::io::Error> {
        queue!(stdout(), Clear(ClearType::All))
    }

    pub fn clear_line() -> Result<(), std::io::Error> {
        queue!(stdout(), Clear(ClearType::CurrentLine))
    }

    pub fn move_cursor_to(position: Position) -> Result<(), std::io::Error> {
        queue!(stdout(), MoveTo(position.x, position.y))
    }

    pub fn print(text: &str) -> Result<(), std::io::Error> {
        queue!(stdout(), crossterm::style::Print(text))
    }

    pub fn show_cursor() -> Result<(), std::io::Error> {
        queue!(stdout(), crossterm::cursor::Show)
    }

    pub fn hide_cursor() -> Result<(), std::io::Error> {
        queue!(stdout(), crossterm::cursor::Hide)
    }

    pub fn execute() -> Result<(), std::io::Error> {
        stdout().flush()
    }
}
