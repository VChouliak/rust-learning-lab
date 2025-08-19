use std::{
    collections::VecDeque,
    sync::{Arc, Mutex, Condvar, atomic::{AtomicBool, Ordering}},
    thread,
};
use ta_core::traits::{Job, ThreadPool};

/// ThreadPool with worker threads, job queue, and synchronization primitives.
pub struct WorkerThreadPool {
    workers: Vec<thread::JoinHandle<()>>,
    queue: Arc<(Mutex<VecDeque<Box<dyn Job>>>, Condvar)>,
    running: Arc<AtomicBool>,
}

impl WorkerThreadPool {
    pub fn new(num_threads: usize) -> Self {
        let queue: Arc<(Mutex<VecDeque<Box<dyn Job>>>, Condvar)> =
            Arc::new((Mutex::new(VecDeque::new()), Condvar::new()));
        let running = Arc::new(AtomicBool::new(true));
        let mut workers = Vec::with_capacity(num_threads);

        for i in 0..num_threads {
            let queue_clone = Arc::clone(&queue);
            let running_clone = Arc::clone(&running);

            let handle = thread::Builder::new()
                .name(format!("worker-{}", i))
                .spawn(move || {
                    loop {
                        let job = {
                            let (lock, cvar) = &*queue_clone;
                            let mut q = lock.lock().unwrap();

                            // Warten bis Job kommt oder Stoppsignal
                            while q.is_empty() && running_clone.load(Ordering::SeqCst) {
                                q = cvar.wait(q).unwrap();
                            }

                            // Wenn keine Jobs mehr und Stoppsignal: Thread beenden
                            if q.is_empty() && !running_clone.load(Ordering::SeqCst) {
                                return;
                            }

                            q.pop_front()
                        };

                        if let Some(job) = job {
                            job.run();
                        }
                    }
                })
                .expect("Failed to spawn worker thread");

            workers.push(handle);
        }

        Self { workers, queue, running }
    }

    pub fn execute<J>(&self, job: J)
    where
        J: Job,
    {
        let (lock, cvar) = &*self.queue;
        let mut q = lock.lock().unwrap();
        q.push_back(Box::new(job));
        cvar.notify_one();
    }

    /// Graceful shutdown by dropping queue and joining threads
    pub fn shutdown(self) {
        self.running.store(false, Ordering::SeqCst);
        let (_, cvar) = &*self.queue;
        cvar.notify_all();

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
        self.execute(job)
    }
}
