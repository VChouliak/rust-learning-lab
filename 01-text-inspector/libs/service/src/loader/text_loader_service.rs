use core::models::text::Content;
use core::traits::loader::Loader;

pub struct Service<'a, TInput>{
    loader: &'a dyn Loader<Content, TInput>,
}

impl <'a, TInput> Service<'a, TInput>{
    pub fn new(loader: &'a dyn Loader<Content, TInput>) -> Self{
        Self{loader}
    }

    pub fn load_content(&self, input: TInput) -> Result<Content, Box<dyn std::error::Error>>{
        self.loader.load(input)
    }
}