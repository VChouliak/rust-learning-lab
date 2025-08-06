pub trait Loader<TOutput, TInput> {
    fn load(&self, source: TInput) ->Result<TOutput, Box<dyn std::error::Error>>;
}