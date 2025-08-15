use ta_core::traits::{Job, ThreadPool};
pub struct AnalyzerService<'a, P: ThreadPool> {
    pool: &'a P,
    chunk_size: usize,
}

impl<'a, P: ThreadPool> AnalyzerService<'a, P> {
    pub fn new(pool: &'a P, chunk_size: usize) -> Self {
        Self {
            pool,
            chunk_size: chunk_size.max(1),
        }
    }

    pub fn process<I>(&self, jobs: I)
    where
        I: IntoIterator<Item = Box<dyn Job>>,
    {
        let mut buffer: Vec<Box<dyn Job>> = Vec::with_capacity(self.chunk_size);
        for job in jobs {
            buffer.push(job);
            if buffer.len() == self.chunk_size {
                self.submit_chunk(&mut buffer);
            }
        }
        if !buffer.is_empty() {
            self.submit_chunk(&mut buffer);
        }
    }

    fn submit_chunk(&self, buffer: &mut Vec<Box<dyn Job>>) {
        let mut group = Vec::with_capacity(buffer.len());
        std::mem::swap( buffer, &mut group);
        self.pool.execute(Box::new(move || {
            for job in group {
                job.run();
            }
        }))
    }
}


