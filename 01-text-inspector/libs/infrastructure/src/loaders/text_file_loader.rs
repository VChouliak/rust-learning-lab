use std::error::Error;
use std::fs;
use std::path::PathBuf;
use core::traits::processor::DataProcessor;
use core::models::text::Content;

pub struct TextFileLoader;

impl DataProcessor<Content, PathBuf> for TextFileLoader {
    fn process(&self, source: PathBuf) -> Result<Content, Box<dyn Error>> {
        let content = fs::read_to_string(&source);
        Ok(Content::new(&content.unwrap().to_string()))
    }
}