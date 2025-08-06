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
    pub line_count: usize,
    pub word_count: usize,
    pub unique_word_count: usize,
    pub average_word_length: f32,
}

impl Statistics {
    pub fn new() -> Statistics {
        Statistics {
            line_count: 0,
            word_count: 0,
            unique_word_count: 0,
            average_word_length: 0.0,
        }
    }
}
