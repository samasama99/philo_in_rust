use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};
use std::sync::mpsc::Sender;
use std::thread;
use crate::forks_pool::ForksPool;
use crate::instructions::Instructions;
use crate::utils::{Fork, MInstant, MPrint};

#[derive(Debug)]
pub struct Philosopher {
    id: u64,
    last_meal: MInstant,
    philosopher_meals: Arc<Mutex<u64>>,
    start_time: Instant,
    instructions: Instructions,
    print_mutex: MPrint,
    left_fork: Fork,
    right_fork: Fork,
    tx: Sender<()>,
    meals: u64,
}

impl Philosopher {
    pub(crate) fn born(
        id: u64,
        (left_fork, right_fork): (Fork, Fork),
        print_mutex: MPrint,
        philosophy_meals: Arc<Mutex<u64>>,
        start_time: Instant,
        instructions: Instructions,
        tx: Sender<()>,
    ) -> Self {
        Self {
            id,
            last_meal: Arc::new(Mutex::new(Instant::now())),
            philosopher_meals: philosophy_meals,
            start_time,
            instructions,
            print_mutex,
            left_fork,
            right_fork,
            tx,
            meals: 0,
        }
    }

    pub(crate) fn start_life(mut self) {
        let instructions = self.instructions;
        let id = self.id;
        let start_time = self.start_time;
        let last_meal_checker = Arc::clone(&self.last_meal);
        let print_mutex_checker = Arc::clone(&self.print_mutex);
        let philosopher_meals_checker = Arc::clone(&self.philosopher_meals);
        let tx_checker = self.tx.clone();

        thread::spawn(move || loop {
            self.eat();
            self.sleep();
            self.think();
        });

        thread::spawn(move || loop {
            thread::sleep(Duration::from_millis(5));
            if instructions.must_eat.is_some()
                && *philosopher_meals_checker.lock().unwrap() == instructions.nums_of_philosophers
            {
                let _print_guard = print_mutex_checker.lock().unwrap();
                tx_checker
                    .send(())
                    .expect("Could not send signal on channel.");
                thread::sleep(Duration::from_secs(u64::MAX));
            };
            if last_meal_checker.lock().unwrap().elapsed() > instructions.time_to_die {
                let _print_guard = print_mutex_checker.lock().unwrap();
                println!("{} philosopher {} is dead ", start_time.elapsed().as_millis(), id);
                tx_checker
                    .send(())
                    .expect("Could not send signal on channel.");
                thread::sleep(Duration::from_secs(u64::MAX));
            }
        });
    }

    fn increment_meals(meals: &mut u64, philosophy_meals: Arc<Mutex<u64>>, instructions: Instructions) {
        if let Some(must_eat) = instructions.must_eat {
            *meals += 1;
            if must_eat == *meals {
                *philosophy_meals.lock().unwrap() += 1;
            }
        };
    }

    fn eat(&mut self) {
        let guard1 = ForksPool::take_fork(&self.left_fork);
        crate::utils::safe_print("took a fork", self.id, self.start_time, &self.print_mutex);
        let guard2 = ForksPool::take_fork(&self.right_fork);
        crate::utils::safe_print("took a fork", self.id, self.start_time, &self.print_mutex);

        self.update_last_meal();

        crate::utils::safe_print("is eating", self.id, self.start_time, &self.print_mutex);

        thread::sleep(self.instructions.time_to_eat);
        Self::increment_meals(
            &mut self.meals,
            Arc::clone(&self.philosopher_meals),
            self.instructions,
        );

        ForksPool::drop_fork(guard1);
        ForksPool::drop_fork(guard2);
    }

    fn sleep(&self) {
        crate::utils::safe_print("is sleeping", self.id, self.start_time, &self.print_mutex);
        thread::sleep(self.instructions.time_to_sleep);
    }

    fn think(&self) {
        crate::utils::safe_print("is thinking", self.id, self.start_time, &self.print_mutex);
    }

    fn update_last_meal(&self) {
        *self.last_meal.lock().unwrap() = Instant::now();
    }
}
