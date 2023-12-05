use std::sync::{Arc, Mutex, MutexGuard};
use crate::utils::Fork;

#[derive(Debug)]
pub struct ForksPool {
    forks: Vec<Fork>,
}

impl ForksPool {
    pub(crate) fn init(nums_of_philosophers: u64) -> Self {
        let forks: Vec<Fork> = (0..nums_of_philosophers)
            .map(|_| Arc::new(Mutex::new(())))
            .collect();
        ForksPool { forks }
    }
    pub(crate) fn get(&self, id: u64) -> Fork {
        Arc::clone(&self.forks[id as usize])
    }

    pub(crate) fn take_fork(fork: &Fork) -> MutexGuard<()> {
        fork.lock().unwrap()
    }

    pub(crate) fn drop_fork(fork_guard: MutexGuard<()>) {
        drop(fork_guard)
    }
}
