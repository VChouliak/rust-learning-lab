use super::job::Job;

pub trait ThreadPool {
    fn execute<J>(&self, job: J)
    where
        J: Job;
}
