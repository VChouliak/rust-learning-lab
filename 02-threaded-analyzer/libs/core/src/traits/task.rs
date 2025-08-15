pub trait Job: Send + 'static {
    fn run(self:Box<Self>);
}

impl <F: FnOnce() + Send + 'static> Job for F {
    fn run(self:Box<Self>) {
        (*self)()
    }
}