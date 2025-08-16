use std::sync::{Arc, Mutex};
use ta_core::traits::ThreadPool;
use infrastructure::threads::WorkerThreadPool;

#[test]
fn thread_pool_executes_jobs() {
    let pool = WorkerThreadPool::new(4);
    let result = Arc::new(Mutex::new(Vec::new()));

    for i in 0..10 {
        let result_clone = Arc::clone(&result);
        pool.execute(Box::new(move || {
            let mut data = result_clone.lock().unwrap();
            data.push(i);
        }));
    }

    // give threads some time to finish
    std::thread::sleep(std::time::Duration::from_millis(500));

    let mut data = result.lock().unwrap();
    data.sort();
    assert_eq!(*data, (0..10).collect::<Vec<_>>());
}
