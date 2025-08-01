use super::editorcommand::{Direction, EditorCommand};
use super::terminal::{Size, Terminal};
use std::cmp::{max, min};

mod buffer;

use crate::editor::position::Position;
use buffer::Buffer;

pub struct View {
    buffer: Buffer,
    needs_redraw: bool,
    size: Size,
    cursor_position: Position,
    scroll_offset: Position,
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
        // we need to ensure that the cursor is always in view
        self.scroll_offset = self.update_scroll_offset(to);
        self.needs_redraw = true;
    }

    pub fn handle_command(&mut self, command: EditorCommand) {
        match command {
            EditorCommand::Move(direction) => self.move_cursor(&direction),
            EditorCommand::Resize(size) => self.resize(size),
            EditorCommand::Quit => {}
        }
    }

    pub fn move_cursor(&mut self, direction: &Direction) {
        let size = Terminal::size().unwrap_or_default();
        self.cursor_position = self.update_cursor_position(direction);
        self.scroll_offset = self.update_scroll_offset(size);
        self.needs_redraw = true;
    }

    pub fn get_cursor_position(&self) -> Position {
        let Position { col, row } = self.cursor_position;
        let offset = self.scroll_offset;

        Position {
            col: col.saturating_sub(offset.col),
            row: row.saturating_sub(offset.row),
        }
    }

    fn update_cursor_position(&self, direction: &Direction) -> Position {
        let Position { mut row, mut col } = self.cursor_position.clone();
        match direction {
            Direction::Left => {
                col = col.saturating_sub(1);
            }
            Direction::Right => {
                col = col.saturating_add(1);
            }
            Direction::Up => {
                row = row.saturating_sub(1);
            }
            Direction::Down => {
                row = row.saturating_add(1);
            }
            Direction::Home => {
                col = 0;
            }
            Direction::End => {
                col = self.buffer.line_len(row).saturating_sub(1);
            }
            Direction::PageUp => {
                row = 0;
            }
            Direction::PageDown => {
                row = self.buffer.num_lines().saturating_sub(1);
            }
        }
        Position { col, row }
    }

    fn update_scroll_offset(&self, size: Size) -> Position {
        // we need to ensure that the cursor is always in view
        let Size { height, width } = size;
        let Position { col, row } = self.cursor_position;

        // Two conditions:
        // (1): dy < row
        // (2): dy + height > row
        let dy = max(
            min(self.scroll_offset.row, row),
            row.saturating_sub(height.saturating_sub(1)),
        );
        // Two conditions:
        // (1): dx < col
        // (2): dx + width > col
        let dx = max(
            min(self.scroll_offset.col, col),
            col.saturating_sub(width.saturating_sub(1)),
        );

        Position { col: dx, row: dy }
    }

    fn render_buffer(&mut self) {
        let Size { height, width } = self.size;
        let Position { col, row } = self.scroll_offset;

        for current in 0..height {
            if let Some(line) = self.buffer.get_line(current + row) {
                let truncated_line = if line.len() > width {
                    &line[col..(col + width)]
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
            cursor_position: Position { col: 0, row: 0 },
            scroll_offset: Position { col: 0, row: 0 },
        }
    }
}
