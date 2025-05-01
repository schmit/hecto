use std::cmp::min;
use crossterm::event::KeyCode;

use crate::editor::position::Position;
use crate::editor::terminal::{Size};


#[derive(Copy, Clone, Default)]
struct Offset {
    dx: usize,
    dy: usize,
}


#[derive(Default)]
pub struct Buffer {
    lines: Vec<String>,
    cursor_position: Position,
    scroll_offset: Offset,
}

impl Buffer {
    pub fn get(&self, index: usize) -> Option<&String> {
        self.lines.get(index)
    }

    pub fn is_empty(&self) -> bool {
        self.lines.is_empty()
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
        let Position { mut row, mut col } = self.cursor_position;
        let Size { height, width } = size;
        match key_code {
            KeyCode::Left => {
                col = col.saturating_sub(1);
            }
            KeyCode::Right => {
                col = min(col.saturating_add(1), width.saturating_sub(1));
            }
            KeyCode::Up => {
                row = row.saturating_sub(1);
            }
            KeyCode::Down => {
                row = min(row.saturating_add(1), height.saturating_sub(1));
            }
            KeyCode::Home => {
                col = 0;
            }
            KeyCode::End => {
                col = width.saturating_sub(1);
            }
            KeyCode::PageUp => {
                row = 0;
            }
            KeyCode::PageDown => {
                row = height.saturating_sub(1);
            }
            _ => (),
        }
        self.cursor_position = Position { col, row };
    }

    pub fn get_cursor_position(&self) -> Position {
        self.cursor_position
    }

}
