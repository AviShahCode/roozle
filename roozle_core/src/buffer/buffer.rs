use std::fs;
use std::ops::Deref;

#[derive(Debug)]
pub struct Buffer {
    content: String,
    // line_starts: Option<Vec<usize>>,
}

// TODO: HashMap<word, count>; preprocessed, word related scan
impl Buffer {
    pub fn new() -> Self {
        Buffer {
            content: String::new(),
            // line_starts: None,
        }
    }

    pub fn from_file(file_name: &str) -> Result<Self, std::io::Error> {
        let content = fs::read_to_string(file_name)?;
        Ok(Buffer { content })
    }

    pub fn from_file_unchecked(file_name: &str) -> Result<Self, std::io::Error> {
        let bytes = fs::read(file_name)?;
        let content = unsafe { str::from_utf8_unchecked(&bytes) };
        Ok(Buffer {
            content: content.to_string(),
        })
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

    pub fn slice(&self, start: usize, end: usize) -> &str {
        &self.content[start..end]
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
