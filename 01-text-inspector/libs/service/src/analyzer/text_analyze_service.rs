use core::models::text::{TextContent, Statistics, Filter};
use core::traits::processor::DataProcessor;
use std::collections::HashSet;

pub struct TextAnalyzer {
    pub filter: Option<Filter>,
}

impl TextAnalyzer {
    pub fn new(filter: Option<Filter>) -> Self {
        Self { filter }
    }
}

impl DataProcessor<Statistics, &TextContent> for TextAnalyzer {
    fn process(&self, input: &TextContent) -> Result<Statistics, Box<dyn std::error::Error>> {
        let lines: Vec<&str> = input
            .lines()
            .filter(|line| {
                if let Some(ref f) = self.filter {
                    f.matches(line)
                } else {
                    true
                }
            })
            .collect();

        let lines_count = lines.len();
        let mut words_count = 0;
        let mut total_word_length = 0;
        let mut unique_words = HashSet::new();

        for line in &lines {
            let words = line.split_whitespace();
            for word in words {
                words_count += 1;
                total_word_length += word.len();
                unique_words.insert(word.to_lowercase());
            }
        }

        let average_word_length = if words_count > 0 {
            total_word_length as f64 / words_count as f64
        } else {
            0.0
        };

        Ok(Statistics {
            lines_count,
            words_count,
            unique_word_count: unique_words.len(),
            average_word_length,
        })
    }
}
