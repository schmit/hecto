use super::terminal::{Position, Size, Terminal};

mod buffer;
use buffer::Buffer;

#[derive(Default)]
pub struct View {
    buffer: Buffer
}

const NAME: &str = env!("CARGO_PKG_NAME");
const VERSION: &str = env!("CARGO_PKG_VERSION");



impl View {
    pub fn render(&self) -> Result<(), std::io::Error> {
        self.draw_rows()?;
        Self::welcome_message()?;
        Ok(())
    }

    fn draw_empty_row() -> Result<(), std::io::Error> {
        Terminal::print("\r\n")?;
        Ok(())
    }

    fn draw_rows(&self) -> Result<(), std::io::Error> {
        let Size { height, .. } = Terminal::size()?;
        for current_row in 0..height {
            Terminal::clear_line()?;
            if let Some(line) = self.buffer.get(current_row) {
                Terminal::print(line)?;
            } else {
                Terminal::print("~")?;
            }
            if current_row.saturating_add(1) < height {
                Self::draw_empty_row()?;
            }
        }

        Ok(())
    }

    pub fn welcome_message() -> Result<(), std::io::Error> {
        let Size { width, height } = Terminal::size()?;
        let mut message = format!("{NAME} editor -- v{VERSION}");
        message.truncate(width);
        let col = width.saturating_sub(message.len()) / 2;
        let row = height / 3;
        Terminal::move_cursor_to(Position { col, row })?;
        Terminal::print(&message)?;
        Ok(())
    }
}

