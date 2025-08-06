pub struct Content {
    pub content: String,
}

impl Content {
    pub fn new(content: &str) -> Content {
        Content {
            content: content.to_string(),
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
