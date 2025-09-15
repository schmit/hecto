use super::line::Line;
use crate::editor::position::Position;

#[derive(Default)]
pub struct Buffer {
    lines: Vec<Line>,
}

impl Buffer {
    pub fn get_line(&self, index: usize) -> Option<&Line> {
        self.lines.get(index)
    }

    pub fn is_empty(&self) -> bool {
        self.lines.is_empty()
    }

    pub fn push(&mut self, line: &str) {
        self.lines.push(Line::from(line));
    }

    pub fn insert(&mut self, at: Position, ch: char) {
        if at.row == self.lines.len() {
            // inserting new line
            self.lines.push(Line::from(&ch.to_string()));
        } else if let Some(line) = self.lines.get_mut(at.row) {
            line.insert(at.col, ch);
        }
    }

    pub fn delete(&mut self, at: Position) -> bool {
        if let Some(line) = self.lines.get_mut(at.row) {
            return line.delete(at.col);
        }
        false
    }

    pub fn load(file_name: &str) -> Result<Self, std::io::Error> {
        let contents = std::fs::read_to_string(file_name)?;
        let mut lines = Vec::new();
        for line in contents.lines() {
            lines.push(Line::from(line));
        }
        Ok(Self { lines })
    }

    pub fn num_lines(&self) -> usize {
        self.lines.len()
    }

    pub fn line_len(&self, at: usize) -> usize {
        let line = self.lines.get(at);
        line.map(|line| line.len()).unwrap_or(0)
    }

    /// Convert a grapheme-based location (line and column) into a
    /// position on the rendered grid, where each grapheme may span
    /// multiple cells.
    pub fn grid_position_of(&self, location: Position) -> Position {
        let Position { row, col } = location;
        let col = self
            .lines
            .get(row)
            .map(|line| line.position_of(col))
            .unwrap_or(0);
        Position { row, col }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::{File, remove_file};
    use std::io::Write;
    use std::time::{SystemTime, UNIX_EPOCH};

    fn unique_file_path() -> std::path::PathBuf {
        let mut path = std::env::temp_dir();
        let nanos = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_nanos();
        path.push(format!("hecto_test_buffer_load_{nanos}"));
        path
    }

    #[test]
    fn load_returns_buffer_with_file_contents() -> std::io::Result<()> {
        let path = unique_file_path();
        let mut file = File::create(&path)?;
        write!(file, "first\nsecond")?;
        drop(file);

        let buffer = Buffer::load(path.to_str().unwrap())?;
        assert_eq!(buffer.num_lines(), 2);
        assert_eq!(
            buffer.get_line(0).map(|line| line.get(0..5)),
            Some("first".to_string())
        );
        assert_eq!(
            buffer.get_line(1).map(|line| line.get(0..6)),
            Some("second".to_string())
        );

        remove_file(path)?;
        Ok(())
    }

    #[test]
    fn load_returns_error_for_missing_file() {
        let path = unique_file_path();
        let result = Buffer::load(path.to_str().unwrap());
        assert!(result.is_err());
    }

    #[test]
    fn push_line() {
        let mut buffer = Buffer::default();
        buffer.push("Hello world!");
        assert!(buffer.num_lines() == 1);
    }

    #[test]
    fn insert_into_new_empty_buffer_creates_line() {
        let mut buffer = Buffer::default();
        buffer.insert(Position { row: 0, col: 0 }, 'A');
        assert_eq!(buffer.num_lines(), 1);
        let line = buffer.get_line(0).unwrap();
        let full_width = line.position_of(line.len());
        assert_eq!(line.get(0..full_width), "A");
    }

    #[test]
    fn insert_into_existing_line_middle() {
        let mut buffer = Buffer::default();
        buffer.push("Helo");
        buffer.insert(Position { row: 0, col: 2 }, 'l');
        let line = buffer.get_line(0).unwrap();
        let full_width = line.position_of(line.len());
        assert_eq!(line.get(0..full_width), "Hello");
    }

    #[test]
    fn insert_at_line_end_appends() {
        let mut buffer = Buffer::default();
        buffer.push("Hello");
        let end = buffer.line_len(0);
        buffer.insert(Position { row: 0, col: end }, '!');
        let line = buffer.get_line(0).unwrap();
        let full_width = line.position_of(line.len());
        assert_eq!(line.get(0..full_width), "Hello!");
    }

    #[test]
    fn insert_col_beyond_end_appends() {
        let mut buffer = Buffer::default();
        buffer.push("Hi");
        buffer.insert(Position { row: 0, col: 100 }, '!');
        let line = buffer.get_line(0).unwrap();
        let full_width = line.position_of(line.len());
        assert_eq!(line.get(0..full_width), "Hi!");
    }

    #[test]
    fn insert_row_beyond_len_is_noop() {
        let mut buffer = Buffer::default();
        buffer.push("Hello");
        buffer.insert(Position { row: 2, col: 0 }, 'X');
        // Row beyond len: should not change existing content or add lines
        assert_eq!(buffer.num_lines(), 1);
        let line = buffer.get_line(0).unwrap();
        let full_width = line.position_of(line.len());
        assert_eq!(line.get(0..full_width), "Hello");
    }

    #[test]
    fn delete_at_line_start() {
        let mut buffer = Buffer::default();
        buffer.push("Hello");
        let deleted = buffer.delete(Position { row: 0, col: 0 });
        assert!(deleted);
        let line = buffer.get_line(0).unwrap();
        let full_width = line.position_of(line.len());
        assert_eq!(line.get(0..full_width), "ello");
        assert_eq!(line.len(), 4);
    }

    #[test]
    fn delete_in_line_middle() {
        let mut buffer = Buffer::default();
        buffer.push("Hxllo");
        let deleted = buffer.delete(Position { row: 0, col: 2 });
        assert!(deleted);
        let line = buffer.get_line(0).unwrap();
        let full_width = line.position_of(line.len());
        assert_eq!(line.get(0..full_width), "Hxlo");
        assert_eq!(line.len(), 4);
    }

    #[test]
    fn delete_at_line_end() {
        let mut buffer = Buffer::default();
        buffer.push("Hello!");
        let last = buffer.line_len(0) - 1;
        let deleted = buffer.delete(Position { row: 0, col: last });
        assert!(deleted);
        let line = buffer.get_line(0).unwrap();
        let full_width = line.position_of(line.len());
        assert_eq!(line.get(0..full_width), "Hello");
        assert_eq!(line.len(), 5);
    }

    #[test]
    fn delete_col_beyond_end_noop() {
        let mut buffer = Buffer::default();
        buffer.push("Hello");
        let deleted = buffer.delete(Position { row: 0, col: 100 });
        assert!(!deleted);
        let line = buffer.get_line(0).unwrap();
        let full_width = line.position_of(line.len());
        assert_eq!(line.get(0..full_width), "Hello");
        assert_eq!(line.len(), 5);
    }

    #[test]
    fn delete_row_beyond_len_noop() {
        let mut buffer = Buffer::default();
        buffer.push("Hello");
        let deleted = buffer.delete(Position { row: 2, col: 0 });
        assert!(!deleted);
        assert_eq!(buffer.num_lines(), 1);
        let line = buffer.get_line(0).unwrap();
        let full_width = line.position_of(line.len());
        assert_eq!(line.get(0..full_width), "Hello");
    }

    #[test]
    fn delete_wide_grapheme_in_line() {
        let mut buffer = Buffer::default();
        buffer.push("a👋b");
        // [a, 👋, b] => delete the wide grapheme at col 1
        let line_before = buffer.get_line(0).unwrap();
        let full_width_before = line_before.position_of(line_before.len());
        assert_eq!(full_width_before, 4);

        let deleted = buffer.delete(Position { row: 0, col: 1 });
        assert!(deleted);

        let line_after = buffer.get_line(0).unwrap();
        let full_width_after = line_after.position_of(line_after.len());
        assert_eq!(full_width_after, 2);
        assert_eq!(line_after.get(0..full_width_after), "ab");
        assert_eq!(line_after.len(), 2);
    }

    #[test]
    fn delete_on_empty_buffer_noop() {
        let mut buffer = Buffer::default();
        let deleted = buffer.delete(Position { row: 0, col: 0 });
        assert!(!deleted);
        assert_eq!(buffer.num_lines(), 0);
    }
}
