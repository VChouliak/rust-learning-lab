use std::sync::{Arc, Mutex};

use ta_core::traits::{Job, ThreadPool};
use service::analyzer::AnalyzerService;

struct FakePool;
impl ThreadPool for FakePool {
    fn execute<J>(&self, job: J)
    where
        J: Job,
    {
        let boxed: Box<dyn Job> = Box::new(job);
        boxed.run();
    }
}

#[test]
fn processes_jobs_in_order_with_chunking() {
    let pool = FakePool;
    let service = AnalyzerService::new(&pool, 2);

    let results = Arc::new(Mutex::new(Vec::<i32>::new()));

    let mut jobs: Vec<Box<dyn Job>> = Vec::new();
    for n in 1..=5 {
        let out = Arc::clone(&results);
        jobs.push(Box::new(move || {
            out.lock().unwrap().push(n);
        }));
    }

    service.process(jobs);

    let got = results.lock().unwrap().clone();
    assert_eq!(got, vec![1, 2, 3, 4, 5]);
}
