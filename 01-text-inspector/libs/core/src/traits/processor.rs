pub trait DataProcessor<TOutput, TInput> {
    fn process(&self, source: TInput) ->Result<TOutput, Box<dyn std::error::Error>>;
}