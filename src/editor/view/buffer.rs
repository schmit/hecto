#[derive(Default)]
pub struct Buffer {
    lines: Vec<String>
}

impl Buffer {
    pub fn get(&self, index: usize) -> Option<&String> {
        self.lines.get(index)
    }

    pub fn push(&mut self, line: String) {
        self.lines.push(line);
    }

    pub fn is_empty(&self) -> bool {
        self.lines.is_empty()
    }
}
