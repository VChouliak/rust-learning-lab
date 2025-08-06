use core::models::text::Content;
use core::traits::processor::DataProcessor;

pub struct Service<'a, TInput>{
    loader: &'a dyn DataProcessor<Content, TInput>,
}

impl <'a, TInput> Service<'a, TInput>{
    pub fn new(loader: &'a dyn DataProcessor<Content, TInput>) -> Self{
        Self{loader}
    }

    pub fn load_content(&self, input: TInput) -> Result<Content, Box<dyn std::error::Error>>{
        self.loader.process(input)
    }
}