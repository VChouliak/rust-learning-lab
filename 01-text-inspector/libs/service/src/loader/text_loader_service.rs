use core::models::text::TextContent;
use core::traits::processor::DataProcessor;

pub struct Service<'a, TInput>{
    loader: &'a dyn DataProcessor<TextContent, TInput>,
}

impl <'a, TInput> Service<'a, TInput>{
    pub fn new(loader: &'a dyn DataProcessor<TextContent, TInput>) -> Self{
        Self{loader}
    }

    pub fn load_content(&self, input: TInput) -> Result<TextContent, Box<dyn std::error::Error>>{
        self.loader.process(input)
    }
}