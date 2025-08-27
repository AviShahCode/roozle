use std::fs;
use std::ops::Deref;

#[derive(Debug)]
pub struct Buffer {
    content: String,
}

impl Buffer {
    pub fn new() -> Self {
        Buffer {
            content: String::new(),
        }
    }

    pub fn from_file(file_name: &str) -> Result<Self, std::io::Error> {
        let content = fs::read_to_string(file_name)?;
        Ok(Buffer { content })
    }

    pub fn from_string(string: &str) -> Self {
        Buffer {
            content: string.to_string(),
        }
    }
    
    pub fn push_string(&mut self, string: &str) {
        self.content.push_str(string);
    }

    pub fn push_line(&mut self, string: &str) {
        self.content.push_str(string);
        self.content.push_str("\n");
    }

    pub fn word_count(&self) -> usize {
        self.content.split_whitespace().count()
    }

    pub fn line_count(&self) -> usize {
        self.content.lines().count()
    }
}

impl Deref for Buffer {
    type Target = str;
    fn deref(&self) -> &Self::Target {
        &self.content
    }
}
