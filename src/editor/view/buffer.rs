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

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::{remove_file, File};
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
        assert_eq!(buffer.get_line(0).map(String::as_str), Some("first"));
        assert_eq!(buffer.get_line(1).map(String::as_str), Some("second"));

        remove_file(path)?;
        Ok(())
    }

    #[test]
    fn load_returns_error_for_missing_file() {
        let path = unique_file_path();
        let result = Buffer::load(path.to_str().unwrap());
        assert!(result.is_err());
    }
}
