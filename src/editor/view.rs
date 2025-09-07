use super::editorcommand::{Direction, EditorCommand};
use super::terminal::{Size, Terminal};
use std::cmp::{max, min};

mod buffer;
mod line;

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
    pub fn new(size: Size) -> Self {
        Self {
            buffer: Buffer::default(),
            needs_redraw: true,
            size,
            cursor_position: Position { col: 0, row: 0 },
            scroll_offset: Position { col: 0, row: 0 },
        }
    }
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
        self.cursor_position = self.update_cursor_position(direction);
        self.scroll_offset = self.update_scroll_offset(self.size);
        self.needs_redraw = true;
    }

    pub fn get_cursor_position(&self) -> Position {
        let absolute = self.buffer.grid_position_of(self.cursor_position);
        let offset = self.scroll_offset;
        Position {
            col: absolute.col.saturating_sub(offset.col),
            row: absolute.row.saturating_sub(offset.row),
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
                // Caret at end: allow position after last grapheme
                col = self.buffer.line_len(row);
            }
            Direction::PageUp => {
                row = row.saturating_sub(self.size.height);
            }
            Direction::PageDown => {
                row = row.saturating_add(self.size.height);
            }
        }
        // Ensure we do not go out of bounds. Allow caret at end of line.
        row = min(self.buffer.num_lines().saturating_sub(1), row);
        col = min(self.buffer.line_len(row), col);
        Position { col, row }
    }

    fn update_scroll_offset(&self, size: Size) -> Position {
        // we need to ensure that the cursor is always in view
        let Size { height, width } = size;
        let Position { row, col } = self.cursor_position;
        let position = self.buffer.grid_position_of(Position { row, col });

        // Two conditions:
        // (1): dy < row
        // (2): dy + height > row
        let dy = max(
            min(self.scroll_offset.row, row),
            row.saturating_sub(height.saturating_sub(1)),
        );
        // Two conditions:
        // (1): dx < col_pos
        // (2): dx + width > col_pos
        let dx = max(
            min(self.scroll_offset.col, position.col),
            position.col.saturating_sub(width.saturating_sub(1)),
        );

        Position { col: dx, row: dy }
    }

    fn render_buffer(&mut self) -> Result<(), std::io::Error> {
        let Size { height, width } = self.size;
        let Position { col, row } = self.scroll_offset;

        for current in 0..height {
            if let Some(line) = self.buffer.get_line(current + row) {
                View::render_line(current, &line.get(col..(col + width)))?;
            } else {
                View::render_line(current, "~")?;
            }
        }
        self.needs_redraw = false;
        Ok(())
    }

    fn render_line(at: usize, line: &str) -> Result<(), std::io::Error> {
        Terminal::print_row(at, line)
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

#[cfg(test)]
mod tests {
    use super::*;

    fn setup() -> View {
        let mut view = View::default();
        view.size = Size {
            width: 5,
            height: 3,
        };
        view.buffer.push("Hello world!");
        view.buffer.push("How are we all doing?");
        view.buffer.push("");
        view.buffer.push("👋Ｂ👋");
        view.buffer.push("Goodbye all");
        view
    }

    #[test]
    fn move_home_direction() {
        let mut view = setup();
        view.cursor_position = Position { row: 1, col: 3 };

        view.move_cursor(&Direction::Home);
        let expected_position = Position { row: 1, col: 0 };
        let expected_offset = Position { row: 0, col: 0 };

        assert_eq!(view.cursor_position, expected_position);
        assert_eq!(view.scroll_offset, expected_offset);
    }

    #[test]
    fn move_end_direction() {
        let mut view = setup();
        view.cursor_position = Position { row: 1, col: 3 };

        view.move_cursor(&Direction::End);
        let expected_position = Position { row: 1, col: 21 };
        let expected_offset = Position { row: 0, col: 17 };

        assert_eq!(view.cursor_position, expected_position);
        assert_eq!(view.scroll_offset, expected_offset);
    }

    #[test]
    fn move_right_cannot_go_past_line() {
        let mut view = setup();
        view.cursor_position = Position { row: 3, col: 2 };
        view.scroll_offset = Position { row: 1, col: 0 };

        view.move_cursor(&Direction::Right);
        let expected_position = Position { row: 3, col: 3 };
        let expected_offset = Position { row: 1, col: 2 };

        assert_eq!(view.cursor_position, expected_position);
        assert_eq!(view.scroll_offset, expected_offset);
    }

    #[test]
    fn move_left_cannot_go_past_line() {
        let mut view = setup();
        view.cursor_position = Position { row: 3, col: 0 };
        view.scroll_offset = Position { row: 1, col: 0 };
        view.move_cursor(&Direction::Left);

        let expected_position = Position { row: 3, col: 0 };
        let expected_offset = Position { row: 1, col: 0 };

        assert_eq!(view.cursor_position, expected_position);
        assert_eq!(view.scroll_offset, expected_offset);
    }

    #[test]
    fn move_left_moves_offset() {
        let mut view = setup();
        view.cursor_position = Position { row: 1, col: 8 };
        view.scroll_offset = Position { row: 1, col: 7 };

        view.move_cursor(&Direction::Left);
        let expected_position = Position { row: 1, col: 7 };
        let expected_offset = Position { row: 1, col: 7 };
        assert_eq!(view.cursor_position, expected_position);
        assert_eq!(view.scroll_offset, expected_offset);

        view.move_cursor(&Direction::Left);
        let expected_position = Position { row: 1, col: 6 };
        let expected_offset = Position { row: 1, col: 6 };
        assert_eq!(view.cursor_position, expected_position);
        assert_eq!(view.scroll_offset, expected_offset);
    }

    #[test]
    fn move_right_moves_offset() {
        let mut view = setup();
        view.cursor_position = Position { row: 1, col: 8 };
        view.scroll_offset = Position { row: 1, col: 5 };

        view.move_cursor(&Direction::Right);
        let expected_position = Position { row: 1, col: 9 };
        let expected_offset = Position { row: 1, col: 5 };
        assert_eq!(
            view.cursor_position, expected_position,
            "expected position 1"
        );
        assert_eq!(view.scroll_offset, expected_offset, "expected offset 1");

        view.move_cursor(&Direction::Right);
        let expected_position = Position { row: 1, col: 10 };
        let expected_offset = Position { row: 1, col: 6 };
        assert_eq!(
            view.cursor_position, expected_position,
            "expected position 2"
        );
        assert_eq!(view.scroll_offset, expected_offset, "expected offset 2");

        view.move_cursor(&Direction::Right);
        let expected_position = Position { row: 1, col: 11 };
        let expected_offset = Position { row: 1, col: 7 };
        assert_eq!(
            view.cursor_position, expected_position,
            "expected position 3"
        );
        assert_eq!(view.scroll_offset, expected_offset, "expected offset 3");
    }

    #[test]
    fn move_up_moves_offset() {
        let mut view = setup();
        view.cursor_position = Position { row: 4, col: 0 };
        view.scroll_offset = Position { row: 3, col: 0 };

        view.move_cursor(&Direction::Up);
        let expected_position = Position { row: 3, col: 0 };
        let expected_offset = Position { row: 3, col: 0 };
        assert_eq!(
            view.cursor_position, expected_position,
            "expected position 1"
        );
        assert_eq!(view.scroll_offset, expected_offset, "expected offset 1");

        view.move_cursor(&Direction::Up);
        let expected_position = Position { row: 2, col: 0 };
        let expected_offset = Position { row: 2, col: 0 };
        assert_eq!(
            view.cursor_position, expected_position,
            "expected position 2"
        );
        assert_eq!(view.scroll_offset, expected_offset, "expected offset 2");
    }

    #[test]
    fn move_down_moves_offset() {
        let mut view = setup();
        view.cursor_position = Position { row: 2, col: 0 };
        view.scroll_offset = Position { row: 1, col: 0 };

        view.move_cursor(&Direction::Down);
        let expected_position = Position { row: 3, col: 0 };
        let expected_offset = Position { row: 1, col: 0 };
        assert_eq!(
            view.cursor_position, expected_position,
            "expected position 1"
        );
        assert_eq!(view.scroll_offset, expected_offset, "expected offset 1");

        view.move_cursor(&Direction::Down);
        let expected_position = Position { row: 4, col: 0 };
        let expected_offset = Position { row: 2, col: 0 };
        assert_eq!(
            view.cursor_position, expected_position,
            "expected position 2"
        );
        assert_eq!(view.scroll_offset, expected_offset, "expected offset 2");
    }

    #[test]
    fn move_down_doesnt_go_below_end() {
        let mut view = setup();
        view.cursor_position = Position { row: 3, col: 1 };
        view.scroll_offset = Position { row: 3, col: 0 };

        view.move_cursor(&Direction::Down);
        let expected_position = Position { row: 4, col: 1 };
        let expected_offset = Position { row: 3, col: 0 };
        assert_eq!(
            view.cursor_position, expected_position,
            "expected position 1"
        );
        assert_eq!(view.scroll_offset, expected_offset, "expected offset 1");

        view.move_cursor(&Direction::Down);
        let expected_position = Position { row: 4, col: 1 };
        let expected_offset = Position { row: 3, col: 0 };
        assert_eq!(
            view.cursor_position, expected_position,
            "expected position 1"
        );
        assert_eq!(view.scroll_offset, expected_offset, "expected offset 1");
    }

    #[test]
    fn move_pageup_subtract_height() {
        let mut view = setup();
        view.cursor_position = Position { row: 5, col: 0 };
        view.scroll_offset = Position { row: 3, col: 0 };

        view.move_cursor(&Direction::PageUp);
        let expected_position = Position { row: 2, col: 0 };
        let expected_offset = Position { row: 2, col: 0 };
        assert_eq!(
            view.cursor_position, expected_position,
            "expected position 1"
        );
        assert_eq!(view.scroll_offset, expected_offset, "expected offset 1");

        view.move_cursor(&Direction::PageUp);
        let expected_position = Position { row: 0, col: 0 };
        let expected_offset = Position { row: 0, col: 0 };
        assert_eq!(
            view.cursor_position, expected_position,
            "expected position 2"
        );
        assert_eq!(view.scroll_offset, expected_offset, "expected offset 2");
    }

    #[test]
    fn move_pagedown_add_height() {
        let mut view = setup();
        view.cursor_position = Position { row: 0, col: 1 };
        view.scroll_offset = Position { row: 0, col: 0 };

        view.move_cursor(&Direction::PageDown);
        let expected_position = Position { row: 3, col: 1 };
        let expected_offset = Position { row: 1, col: 0 };
        assert_eq!(
            view.cursor_position, expected_position,
            "expected position 1"
        );
        assert_eq!(view.scroll_offset, expected_offset, "expected offset 1");

        view.move_cursor(&Direction::PageDown);
        // note row is 4 because we cannot go out of bounds
        let expected_position = Position { row: 4, col: 1 };
        let expected_offset = Position { row: 2, col: 0 };
        assert_eq!(
            view.cursor_position, expected_position,
            "expected position 2"
        );
        assert_eq!(view.scroll_offset, expected_offset, "expected offset 2");
    }

    #[test]
    fn get_cursor_position_counts_wide_chars() {
        let mut view = View::default();
        view.buffer.push("ＡＢ");
        view.cursor_position = Position { row: 0, col: 1 };

        let pos = view.get_cursor_position();
        assert_eq!(pos, Position { row: 0, col: 2 });
    }

    #[test]
    fn get_cursor_position_handles_zero_width() {
        let mut view = View::default();
        view.buffer.push("a\u{200B}b");
        view.cursor_position = Position { row: 0, col: 1 };

        let pos = view.get_cursor_position();
        assert_eq!(pos, Position { row: 0, col: 1 });
    }
}
