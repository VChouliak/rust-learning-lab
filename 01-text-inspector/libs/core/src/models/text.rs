pub struct TextContent {
    pub raw: String,
}

impl TextContent {
    pub fn new(content: &str) -> TextContent {
        TextContent {
            raw: content.to_string(),
        }
    }
    pub fn lines(&self) -> impl Iterator<Item = &str> {
        self.raw.lines()
    }
}

#[derive(Debug, Clone)]
pub enum Filter {
    Contains(String),
    StartsWith(String),
    EndsWith(String),
}

impl Filter {
    pub fn matches(&self, line: &str) -> bool {
        match self {
            Filter::Contains(term) => line.contains(term),
            Filter::StartsWith(prefix) => line.starts_with(prefix),
            Filter::EndsWith(suffix) => line.ends_with(suffix),
        }
    }
}


pub struct Statistics {
    pub lines_count: usize,
    pub words_count: usize,
    pub unique_word_count: usize,
    pub average_word_length: f64,
}

impl Statistics {
    pub fn new() -> Statistics {
        Statistics {
            lines_count: 0,
            words_count: 0,
            unique_word_count: 0,
            average_word_length: 0.0,
        }
    }
}

