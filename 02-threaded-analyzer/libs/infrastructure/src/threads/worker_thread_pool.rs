use std::{
    collections::VecDeque,
    sync::{Arc, Mutex, Condvar},
    thread::{self},
};

use ta_core::traits::Job;
use ta_core::traits::ThreadPool;

/// ThreadPool with worker threads, job queue, and synchronization primitives.
pub struct WorkerThreadPool {
    workers: Vec<thread::JoinHandle<()>>,
    queue: Arc<(Mutex<VecDeque<Box<dyn Job>>>, Condvar)>,
}

impl WorkerThreadPool {
    pub fn new(num_threads: usize) -> Self {
        let queue:Arc<(Mutex<VecDeque<Box<dyn Job>>>, Condvar)> = Arc::new((Mutex::new(VecDeque::new()), Condvar::new()));
        let mut workers = Vec::with_capacity(num_threads);

        for i in 0..num_threads {
            let queue_clone = Arc::clone(&queue);

            let handle = thread::Builder::new()
                .name(format!("worker-{}", i))
                .spawn(move || loop {
                    let job = {
                        let (lock, cvar) = &*queue_clone;
                        let mut q = lock.lock().unwrap();

                        while q.is_empty() {
                            q = cvar.wait(q).unwrap(); // Park until a job is available
                        }

                        q.pop_front()
                    };

                    if let Some(job) = job {
                        job.run();
                    }
                })
                .expect("Failed to spawn worker thread");

            workers.push(handle);
        }

        Self { workers, queue }
    }

    /// Graceful shutdown by dropping queue and joining threads
    pub fn shutdown(self) {
        // Dropping queue Arc will stop threads eventually (they will panic if not handled)
        for worker in self.workers {
            let _ = worker.join();
        }
    }
}

impl ThreadPool for WorkerThreadPool {
    fn execute<J>(&self, job: J)
    where
        J: Job,
    {
        let (lock, cvar) = &*self.queue;
        let mut q = lock.lock().unwrap();
        q.push_back(Box::new(job));
        cvar.notify_one(); // Wake up one worker
    }
}
