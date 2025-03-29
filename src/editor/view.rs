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
        if self.buffer.is_empty() {
            Self::render_welcome_message()?;
        } else {
            self.render_buffer()?
        }
        Ok(())
    }

    pub fn load(&mut self, file_name: &str) -> Result<(), std::io::Error> {
        let file_contents = std::fs::read_to_string(file_name)?;
        for line in file_contents.lines() {
            self.buffer.push(line.to_string());
        }
        Ok(())
    }

    fn render_buffer(&self) -> Result<(), std::io::Error> {
        let Size { height, .. } = Terminal::size()?;

        for current in  0..height {
            Terminal::clear_line()?;
            if let Some(line) = self.buffer.get(current) {
                Terminal::print(line)?;
                Terminal::print("\r\n")?;
            } else {
                Self::draw_empty_row()?;
                if current.saturating_add(1) < height {
                    Terminal::print("\r\n")?
                }
            }
        }
        Ok(())
    }

    pub fn render_welcome_message() -> Result<(), std::io::Error> {
        let Size { width, height } = Terminal::size()?;
        let mut message = format!("{NAME} editor -- v{VERSION}");
        message.truncate(width);
        let col = width.saturating_sub(message.len()) / 2;
        let row = height / 3;
        Terminal::move_cursor_to(Position { col, row })?;
        Terminal::print(&message)?;
        Ok(())
    }

    fn draw_empty_row() -> Result<(), std::io::Error> {
        Terminal::print("~")?;
        Ok(())
    }

}

