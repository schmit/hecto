#[derive(Default)]
pub struct Buffer {
    lines: Vec<String>,
}

impl Buffer {
    pub fn get_line(&self, index: usize) -> Option<&String> {
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
        Ok(Self { lines })
    }

    pub fn num_lines(&self) -> usize {
        self.lines.len()
    }

    pub fn line_len(&self, line: usize) -> usize {
        self.lines[line].len()
    }
}
