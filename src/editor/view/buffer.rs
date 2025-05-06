use std::cmp::{max, min};
use crossterm::event::KeyCode;

use crate::editor::position::Position;
use crate::editor::terminal::{Size};


#[derive(Copy, Clone, Default)]
pub struct Offset {
    pub dx: usize,
    pub dy: usize,
}


#[derive(Default)]
pub struct Buffer {
    lines: Vec<String>,
    cursor_position: Position,
    scroll_offset: Offset,
}

impl Buffer {
    pub fn get_line(&self, index: usize) -> Option<&String> {
        self.lines.get(index)
    }


    pub fn is_empty(&self) -> bool {
        self.lines.is_empty()
    }

    pub fn get_offset(&self) -> Offset {
        self.scroll_offset
    }

    pub fn load(file_name: &str) -> Result<Self, std::io::Error> {
        // reads contents of a file and
        // returns a new buffer with content in lines
        let contents = std::fs::read_to_string(file_name)?;
        let mut lines = Vec::new();
        for line in contents.lines() {
            lines.push(line.to_string());
        }
        let cursor_position = Position::default();
        let scroll_offset = Offset::default();
        Ok(Self { lines, cursor_position, scroll_offset })
    }

    pub fn move_cursor(&mut self, key_code: KeyCode, size: Size) {
        self.cursor_position = self.update_cursor_position(key_code, size);
        self.scroll_offset = self.update_scroll_offset(size);
    }

    pub fn get_cursor_position(&self) -> Position {
        let Position { col, row } = self.cursor_position;
        let Offset { dx, dy } = self.scroll_offset;
        
        Position { col: col.saturating_sub(dx), row: row.saturating_sub(dy) }
    }

    fn num_lines(&self) -> usize {
        self.lines.len()
    }

    fn line_len(&self, line: usize) -> usize {
        self.lines[line].len()
    }

    fn update_cursor_position(&self, key_code: KeyCode, size: Size) -> Position {
        let Position { mut row, mut col } = self.cursor_position.clone();
        let Size { height, width } = size;
        match key_code {
            KeyCode::Left => {
                col = col.saturating_sub(1);
            }
            KeyCode::Right => {
                col = col.saturating_add(1);
            }
            KeyCode::Up => {
                row = row.saturating_sub(1);
            }
            KeyCode::Down => {
                row = row.saturating_add(1);
            }
            KeyCode::Home => {
                col = 0;
            }
            KeyCode::End => {
                col = self.line_len(row).saturating_sub(1);
            }
            KeyCode::PageUp => {
                row = 0;
            }
            KeyCode::PageDown => {
                row = self.num_lines().saturating_sub(1);
            }
            _ => (),
        }
        Position { col, row }
    }

    fn update_scroll_offset(&self, size: Size) -> Offset {
        // we need to ensure that the cursor is always in view
        let Size { height, width } = size;
        let Position { col, row } = self.cursor_position;

        // Two conditions:
        // (1): dy < row
        // (2): dy + height > row
        let dy = max(min(self.scroll_offset.dy, row), row.saturating_sub(height.saturating_sub(1)));
        // Two conditions:
        // (1): dx < col
        // (2): dx + width > col
        let dx = max(min(self.scroll_offset.dx, col), col.saturating_sub(width.saturating_sub(1)));

        Offset { dx, dy }
    }

}
