use std::cmp;
use std::ops::Range;

pub struct Line {
    string: String,
}

impl Line {
    pub fn from(line_str: &str) -> Self {
        Self { string: String::from(line_str) }
    }

    pub fn get(&self, range: Range<usize>) -> String {
        let start = range.start;
        let end = cmp::min(range.end, self.string.len());
        self.string.get(start..end).unwrap_or_default().to_string()
    }

    pub fn len(&self) -> usize {
        self.string.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty_line() {
        let line = Line::from("");
        assert_eq!(line.get(0..0), "");
        assert_eq!(line.get(0..1), "");
        assert_eq!(line.get(1..1), "");
        assert_eq!(line.get(1..2), "");
    }
    
    #[test]
    fn test_hello_world() {
        let line = Line::from("Hello, world!");
        assert_eq!(line.get(0..0), "");
        assert_eq!(line.get(0..1), "H");
        assert_eq!(line.get(0..12), "Hello, world");
        assert_eq!(line.get(0..13), "Hello, world!");
        assert_eq!(line.get(0..14), "Hello, world!");
        assert_eq!(line.get(1..14), "ello, world!");
        assert_eq!(line.get(14..16), "");
    }
    
    #[test]
    fn test_len() {
        let line = Line::from("Hello, world!");
        assert_eq!(line.len(), 13);
    }
}