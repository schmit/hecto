pub struct Buffer {
    lines: Vec<String>
}

impl Buffer {
    pub fn get(&self, index: usize) -> Option<&String> {
        self.lines.get(index)
    }
}

impl Default for Buffer {
    fn default() -> Self {
        Self {
            lines: vec!["Hello, World!".to_string()],
        }
    }
}
