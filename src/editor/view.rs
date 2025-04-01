use super::terminal::{Position, Size, Terminal};

mod buffer;
use buffer::Buffer;

pub struct View {
    buffer: Buffer,
    needs_redraw: bool,
    size: Size,
}

const NAME: &str = env!("CARGO_PKG_NAME");
const VERSION: &str = env!("CARGO_PKG_VERSION");



impl View {
    pub fn render(&mut self) -> Result<(), std::io::Error> {
        if !self.needs_redraw {
            return Ok(());
        }

        if self.buffer.is_empty() {
            self.render_welcome_message()?;
        } else {
            self.render_buffer()?;

        }
        Ok(())
    }

    pub fn load(&mut self, file_name: &str) -> Result<(), std::io::Error> {
        let file_contents = std::fs::read_to_string(file_name)?;
        for line in file_contents.lines() {
            self.buffer.push(line.to_string());
        }
        self.needs_redraw = true;
        Ok(())
    }

    pub fn resize(&mut self, to: Size) {
        self.size = to;
        self.needs_redraw = true;
    }

    fn render_buffer(&mut self) -> Result<(), std::io::Error> {
        let Size { height, width } = self.size;

        for current in  0..height {
            if let Some(line) = self.buffer.get(current) {
                let truncated_line = if line.len() > width {
                    &line[..width]
                } else {
                    line
                };
                View::render_line(current, truncated_line)?;
            } else {
                View::render_line(current, "~")?;
            }
        }
        self.needs_redraw = false;
        Ok(())
    }

    fn render_line(at: usize, line: &str) -> Result<(), std::io::Error>  {
        Terminal::move_cursor_to(Position { col: 0, row: at })?;
        Terminal::clear_line()?;
        Terminal::print(line)?;
        Ok(())
    }

    pub fn render_welcome_message(&self) -> Result<(), std::io::Error> {
        let Size { width, height } = self.size;
        let mut message = format!("{NAME} editor -- v{VERSION}");
        message.truncate(width);
        let col = width.saturating_sub(message.len()) / 2;
        let row = height / 3;
        Terminal::move_cursor_to(Position { col, row })?;
        Terminal::print(&message)?;
        Ok(())
    }

}

impl Default for View {
    fn default() -> Self {
        Self {
            buffer: Buffer::default(),
            needs_redraw: true,
            size: Terminal::size().unwrap(),
        }
    }
}
