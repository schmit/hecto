use super::terminal::{Size, Terminal};
use super::position::Position;
use crossterm::event::KeyCode;

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
    pub fn render(&mut self) {
        if !self.needs_redraw {
            return;
        }

        if self.buffer.is_empty() {
            let result = self.render_welcome_message();
            debug_assert!(result.is_ok());
        } else {
            self.render_buffer();
        }
    }

    pub fn load(&mut self, file_name: &str) {
        if let Ok(buffer) = Buffer::load(file_name) {
            self.buffer = buffer;
        }
        self.needs_redraw = true;
    }

    pub fn resize(&mut self, to: Size) {
        self.size = to;
        self.needs_redraw = true;
    }

    pub fn move_cursor(&mut self, key_code: KeyCode) {
        let size = Terminal::size().unwrap_or_default();
        self.buffer.move_cursor(key_code, size);
    }

    pub fn refresh_screen(&mut self) {
        let _ = Terminal::hide_cursor();
        self.render();
        let _ = Terminal::move_cursor_to(self.buffer.get_cursor_position());
        let _ = Terminal::show_cursor();
        let _ = Terminal::execute();
    }

    fn render_buffer(&mut self) {
        let Size { height, width } = self.size;

        for current in 0..height {
            if let Some(line) = self.buffer.get(current) {
                let truncated_line = if line.len() > width {
                    &line[..width]
                } else {
                    line
                };
                View::render_line(current, truncated_line);
            } else {
                View::render_line(current, "~");
            }
        }
        self.needs_redraw = false;
    }

    fn render_line(at: usize, line: &str) {
        let result = Terminal::print_row(at, line);
        debug_assert!(result.is_ok(), "Failed to render line");
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
            size: Terminal::size().unwrap_or_default(),
        }
    }
}
