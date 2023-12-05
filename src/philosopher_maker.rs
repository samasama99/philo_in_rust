use std::sync::mpsc::{channel, Receiver, Sender};
use std::time::Instant;
use std::sync::{Arc, Mutex};
use crate::utils::MPrint;
use crate::forks_pool::ForksPool;
use crate::instructions::Instructions;
use crate::philosopher::Philosopher;

#[derive(Debug)]
pub struct PhilosopherMaker {
    nums_of_philosophers: u64,
    forks: ForksPool,
    print_mutex: MPrint,
    philosopher_meals: Arc<Mutex<u64>>,
    start_time: Instant,
    initialised: bool,
    philosopher_made: u64,
    instructions: Instructions,
    tx: Sender<()>,
}

impl PhilosopherMaker {
    pub(crate) fn init(instructions: Instructions) -> (Self, Receiver<()>) {
        let (tx, rx) = channel();

        (
            Self {
                nums_of_philosophers: instructions.nums_of_philosophers,
                forks: ForksPool::init(instructions.nums_of_philosophers),
                start_time: Instant::now(),
                print_mutex: Arc::new(Mutex::new(())),
                philosopher_meals: Arc::new(Mutex::new(0)),
                initialised: true,
                philosopher_made: 0,
                instructions,
                tx,
            },
            rx,
        )
    }
    fn make(&mut self) -> Philosopher {
        assert!(self.initialised);
        self.philosopher_made += 1;
        let id = self.philosopher_made;
        Philosopher::born(
            id,
            (
                self.forks.get(id - 1),
                self.forks.get(id % self.nums_of_philosophers),
            ),
            Arc::clone(&self.print_mutex),
            Arc::clone(&self.philosopher_meals),
            self.start_time,
            self.instructions,
            self.tx.clone(),
        )
    }

    pub(crate) fn start_simulation(mut self) {
        for _ in 0..self.instructions.nums_of_philosophers {
            let philosopher = self.make();
            philosopher.start_life();
        }
    }
}
