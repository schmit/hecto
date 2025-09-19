use crossterm::cursor::MoveTo;
use crossterm::terminal::{
    Clear, ClearType, EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode,
    enable_raw_mode, size,
};
use crossterm::{Command, queue};
use std::io::{Write, stdout};

#[derive(Copy, Clone, Default)]
pub struct Size {
    pub width: usize,
    pub height: usize,
}

use crate::editor::position::Position;

pub struct Terminal {}

impl Terminal {
    pub fn size() -> Result<Size, std::io::Error> {
        let (width_u16, height_u16) = size()?;
        let width = usize::from(width_u16);
        let height = usize::from(height_u16);
        Ok(Size { width, height })
    }

    pub fn initialize() -> Result<(), std::io::Error> {
        enable_raw_mode()?;
        Self::enter_alternate_screen()?;
        Self::clear_screen()?;
        Self::move_cursor_to(Position::default())?;
        Self::flush()?;
        Ok(())
    }

    pub fn terminate() -> Result<(), std::io::Error> {
        Self::leave_alternate_screen()?;
        Self::show_cursor()?;
        Self::flush()?;
        disable_raw_mode()?;
        Ok(())
    }

    fn enter_alternate_screen() -> Result<(), std::io::Error> {
        Self::queue_command(EnterAlternateScreen)?;
        Ok(())
    }

    fn leave_alternate_screen() -> Result<(), std::io::Error> {
        Self::queue_command(LeaveAlternateScreen)?;
        Ok(())
    }

    pub fn clear_screen() -> Result<(), std::io::Error> {
        Self::queue_command(Clear(ClearType::All))
    }

    pub fn clear_line() -> Result<(), std::io::Error> {
        Self::queue_command(Clear(ClearType::CurrentLine))
    }

    pub fn move_cursor_to(position: Position) -> Result<(), std::io::Error> {
        let x = u16::try_from(position.col).unwrap_or(u16::MAX);
        let y = u16::try_from(position.row).unwrap_or(u16::MAX);
        Self::queue_command(MoveTo(x, y))
    }

    pub fn print(string: &str) -> Result<(), std::io::Error> {
        Self::queue_command(crossterm::style::Print(string))
    }

    pub fn print_row(row: usize, line: &str) -> Result<(), std::io::Error> {
        Self::move_cursor_to(Position { col: 0, row })?;
        Self::clear_line()?;
        Self::print(line)?;
        Ok(())
    }

    pub fn begin_frame() -> Result<(), std::io::Error> {
        Self::hide_cursor()
    }

    pub fn end_frame(cursor: Position) -> Result<(), std::io::Error> {
        Self::move_cursor_to(cursor)?;
        Self::show_cursor()?;
        Self::flush()
    }

    pub fn show_cursor() -> Result<(), std::io::Error> {
        Self::queue_command(crossterm::cursor::Show)
    }

    pub fn hide_cursor() -> Result<(), std::io::Error> {
        Self::queue_command(crossterm::cursor::Hide)
    }

    pub fn flush() -> Result<(), std::io::Error> {
        stdout().flush()
    }

    pub fn queue_command<T: Command>(command: T) -> Result<(), std::io::Error> {
        queue!(stdout(), command)?;
        Ok(())
    }
}
