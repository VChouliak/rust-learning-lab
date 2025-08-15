use super::task::Job;

pub trait Executor {
    fn execute<J>(&self, job: J)
    where
        J: Job;
}
