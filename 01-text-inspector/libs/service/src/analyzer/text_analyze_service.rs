use std::collections::HashSet;
use core::models::text::Statistics;
use core::traits::processor::DataProcessor;
pub struct TextAnalyzer;

impl DataProcessor<Statistics, &str> for TextAnalyzer {
    fn process(&self, input: &str) -> Result<Statistics, Box<dyn std::error::Error>> {
        let lines: Vec<&str> = input.lines().collect();
        let lines_count = lines.len();

        let mut words_count = 0;
        let mut total_word_length = 0;
        let mut unique_words = HashSet::new();

        for line in &lines {
            let words: Vec<&str> = line.split_whitespace().collect();
            words_count += words.len();
            for word in &words {
                total_word_length += word.len();
                unique_words.insert(word.to_lowercase());
            }
        }

        let average_word_length = if words_count > 0 {
            total_word_length as f64 / words_count as f64
        }else {
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
