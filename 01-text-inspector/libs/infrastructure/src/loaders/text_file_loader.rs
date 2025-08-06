use std::error::Error;
use std::fs;
use std::path::PathBuf;
use core::traits::processor::DataProcessor;
use core::models::text::TextContent;

pub struct TextFileLoader;

impl DataProcessor<TextContent, PathBuf> for TextFileLoader {
    fn process(&self, source: PathBuf) -> Result<TextContent, Box<dyn Error>> {
        let content = fs::read_to_string(&source);
        Ok(TextContent::new(&content.unwrap().to_string()))
    }
}