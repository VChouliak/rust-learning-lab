use std::sync::{Arc, atomic::{AtomicUsize, Ordering}};
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
        let jobs_iter = jobs.into_iter().collect::<Vec<_>>();
        let total_jobs = jobs_iter.len();
        if total_jobs == 0 { return; }

        let counter = Arc::new(AtomicUsize::new(0));
        let mut buffer: Vec<Box<dyn Job>> = Vec::with_capacity(self.chunk_size);

        for job in jobs_iter {
            buffer.push(job);
            if buffer.len() == self.chunk_size {
                self.submit_chunk(&mut buffer, Arc::clone(&counter));
            }
        }

        if !buffer.is_empty() {
            self.submit_chunk(&mut buffer, Arc::clone(&counter));
        }

        while counter.load(Ordering::SeqCst) < total_jobs {
            std::thread::yield_now();
        }
    }

    fn submit_chunk(&self, buffer: &mut Vec<Box<dyn Job>>, counter: Arc<AtomicUsize>) {
        let mut group = Vec::with_capacity(buffer.len());
        std::mem::swap(buffer, &mut group);

        self.pool.execute(Box::new(move || {
            for job in group {
                job.run();
                counter.fetch_add(1, Ordering::SeqCst);
            }
        }));
    }
}
