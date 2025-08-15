use core::traits::task::Job;
use std::sync::{Arc, Mutex};
#[test]
fn job_updates_string() {
    let greeting = Arc::new(Mutex::new("Hello".to_string()));
    let greeting_clone = greeting.clone();

    let job = Box::new(move || {
        let mut text = greeting_clone.lock().unwrap();
        text.push_str(" World");
    });

    let boxed_job: Box<dyn Job> = job;

    boxed_job.run();

    assert_eq!(&*greeting.lock().unwrap(), "Hello World");
}

#[test]
fn job_runs_closure() {

    let counter = Arc::new(Mutex::new(0));
    let counter_clone = counter.clone();

    let job = Box::new(move || {
        let mut num = counter_clone.lock().unwrap();
        *num += 1;
    });

    let boxed_job: Box<dyn Job> = job;

    boxed_job.run();

    assert_eq!(*counter.lock().unwrap(), 1);
}