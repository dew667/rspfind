pub struct File {
    pub name: String,
    format: Format,
    raw_content: String,
    line_count: usize,
    line_content: Vec<String>,
    line_index: usize,
    line_offset: usize,
}

pub enum Format {
    Text,
    Binary,
}

impl File {
    pub fn new(name: String, format: Format, raw_content: String) -> Self {
        let line_content: Vec<String> = raw_content.lines().map(|s| s.to_string()).collect();
        let line_count = line_content.len();
        File {
            name,
            format,
            raw_content,
            line_count,
            line_content,
            line_index: 0,
            line_offset: 0,
        }
    }

    pub fn next_line(&mut self) -> Option<&String> {
        if self.line_index < self.line_count {
            let line = &self.line_content[self.line_index];
            self.line_index += 1;
            Some(line)
        } else {
            None
        }
    }

    pub fn line_index(&self) -> usize {
        self.line_index
    }
    
}