use std::sync::{Arc, Mutex};

use ta_core::traits::{Job, ThreadPool};
use service::analyzer::AnalyzerService;
use infrastructure::threads::WorkerThreadPool;

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

#[test]
fn analyzer_service_processes_in_chunks() {
    let pool = WorkerThreadPool::new(2);
    let service = AnalyzerService::new(&pool, 3);

    let result = Arc::new(Mutex::new(Vec::new()));

    let jobs: Vec<Box<dyn Job>> = (0..9)
        .map(|i| {
            let result_clone = Arc::clone(&result);
            Box::new(move || {
                let mut data = result_clone.lock().unwrap();
                data.push(i);
            }) as Box<dyn Job>
        })
        .collect();

    service.process(jobs);

    std::thread::sleep(std::time::Duration::from_millis(500));

    let mut data = result.lock().unwrap();
    data.sort();
    assert_eq!(*data, (0..9).collect::<Vec<_>>());
}