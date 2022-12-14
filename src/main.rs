// #![allow(unused)]
use std::env::args;
use std::str::FromStr;
use std::sync::mpsc::{channel, Receiver, Sender};
use std::sync::{Arc, Mutex, MutexGuard};
use std::thread;
use std::time::{Duration, Instant};

type MInstant = Arc<Mutex<Instant>>;

type MPrint = Arc<Mutex<()>>;

type Fork = Arc<Mutex<()>>;

#[derive(Debug)]
struct Philo {
    id: u64,
    last_meal: MInstant,
    philos_meals: Arc<Mutex<u64>>,
    start_time: Instant,
    instructions: Instructions,
    print_mutex: MPrint,
    left_fork: Fork,
    right_fork: Fork,
    tx: Sender<()>,
    meals: u64,
}

impl Philo {
    fn born(
        id: u64,
        (left_fork, right_fork): (Fork, Fork),
        print_mutex: MPrint,
        philos_meals: Arc<Mutex<u64>>,
        start_time: Instant,
        instructions: Instructions,
        tx: Sender<()>,
    ) -> Self {
        Self {
            id,
            last_meal: Arc::new(Mutex::new(Instant::now())),
            philos_meals,
            start_time,
            instructions,
            print_mutex,
            left_fork,
            right_fork,
            tx,
            meals: 0,
        }
    }

    fn start_life(mut self) {
        let instructions = self.instructions;
        let id = self.id;
        let start_time = self.start_time;
        let last_meal_checker = Arc::clone(&self.last_meal);
        let print_mutex_checker = Arc::clone(&self.print_mutex);
        let philos_meals_checker = Arc::clone(&self.philos_meals);
        let tx_checker = self.tx.clone();

        thread::spawn(move || loop {
            self.eat();
            self.sleep();
            self.think();
        });

        thread::spawn(move || loop {
            thread::sleep(Duration::from_millis(5));
            if instructions.must_eat.is_some()
                && *philos_meals_checker.lock().unwrap() == instructions.nums_of_philos
            {
                let _print_guard = print_mutex_checker.lock().unwrap();
                tx_checker
                    .send(())
                    .expect("Could not send signal on channel.");
                thread::sleep(Duration::from_secs(u64::MAX));
            };
            if last_meal_checker.lock().unwrap().elapsed() > instructions.time_to_die {
                let _print_guard = print_mutex_checker.lock().unwrap();
                println!("{} philo {} is dead ", start_time.elapsed().as_millis(), id);
                tx_checker
                    .send(())
                    .expect("Could not send signal on channel.");
                thread::sleep(Duration::from_secs(u64::MAX));
            }
        });
    }

    fn increment_meals(meals: &mut u64, philos_meals: Arc<Mutex<u64>>, instructions: Instructions) {
        if let Some(must_eat) = instructions.must_eat {
            *meals += 1;
            if must_eat == *meals {
                *philos_meals.lock().unwrap() += 1;
            }
        };
    }

    fn eat(&mut self) {
        let guard1 = ForksPool::take_fork(&self.left_fork);
        safe_print("took a fork", self.id, self.start_time, &self.print_mutex);
        let guard2 = ForksPool::take_fork(&self.right_fork);
        safe_print("took a fork", self.id, self.start_time, &self.print_mutex);

        self.update_last_meal();

        safe_print("is eating", self.id, self.start_time, &self.print_mutex);

        thread::sleep(self.instructions.time_to_eat);
        Self::increment_meals(
            &mut self.meals,
            Arc::clone(&self.philos_meals),
            self.instructions,
        );

        ForksPool::drop_fork(guard1);
        ForksPool::drop_fork(guard2);
    }

    fn sleep(&self) {
        safe_print("is sleeping", self.id, self.start_time, &self.print_mutex);
        thread::sleep(self.instructions.time_to_sleep);
    }

    fn think(&self) {
        safe_print("is thinking", self.id, self.start_time, &self.print_mutex);
    }

    fn update_last_meal(&self) {
        *self.last_meal.lock().unwrap() = Instant::now();
    }
}

#[derive(Debug)]
struct PhiloMaker {
    nums_of_philos: u64,
    forks: ForksPool,
    print_mutex: MPrint,
    philos_meals: Arc<Mutex<u64>>,
    start_time: Instant,
    initialised: bool,
    philos_made: u64,
    instructions: Instructions,
    tx: Sender<()>,
}

impl PhiloMaker {
    fn init(instructions: Instructions) -> (Self, Receiver<()>) {
        let (tx, rx) = channel();

        (
            Self {
                nums_of_philos: instructions.nums_of_philos,
                forks: ForksPool::init(instructions.nums_of_philos),
                start_time: Instant::now(),
                print_mutex: Arc::new(Mutex::new(())),
                philos_meals: Arc::new(Mutex::new(0)),
                initialised: true,
                philos_made: 0,
                instructions,
                tx,
            },
            rx,
        )
    }
    fn make(&mut self) -> Philo {
        assert!(self.initialised);
        self.philos_made += 1;
        let id = self.philos_made;
        Philo::born(
            id,
            (
                self.forks.get(id - 1),
                self.forks.get(id % self.nums_of_philos),
            ),
            Arc::clone(&self.print_mutex),
            Arc::clone(&self.philos_meals),
            self.start_time,
            self.instructions,
            self.tx.clone(),
        )
    }

    fn start_simulation(mut self) {
        for _ in 0..self.instructions.nums_of_philos {
            let philo = self.make();
            philo.start_life();
        }
    }
}

#[derive(Debug)]
struct ForksPool {
    forks: Vec<Fork>,
}

impl ForksPool {
    fn init(nums_of_philos: u64) -> Self {
        let forks: Vec<Fork> = (0..nums_of_philos)
            .map(|_| Arc::new(Mutex::new(())))
            .collect();
        ForksPool { forks }
    }
    fn get(&self, id: u64) -> Fork {
        Arc::clone(&self.forks[id as usize])
    }

    fn take_fork(fork: &Fork) -> MutexGuard<()> {
        fork.lock().unwrap()
    }

    fn drop_fork(fork_guard: MutexGuard<()>) {
        drop(fork_guard)
    }
}

#[derive(Debug, Default, Clone, Copy)]
struct Instructions {
    nums_of_philos: u64,
    time_to_die: Duration,
    time_to_eat: Duration,
    time_to_sleep: Duration,
    must_eat: Option<u64>,
}

fn string_to<T: FromStr>(target: &str, err: &str) -> Result<T, String> {
    match target.parse::<T>() {
        Ok(value) => Ok(value),
        _ => Err(err.to_string()),
    }
}

impl Instructions {
    fn new(
        nums_of_philos: &str,
        ttd: &str,
        tte: &str,
        tts: &str,
        must_eat: Option<&String>,
    ) -> Result<Self, String> {
        Ok(Instructions {
            nums_of_philos: string_to::<u64>(nums_of_philos, "error parsing nums of philos")?,
            time_to_die: Duration::from_millis(string_to(ttd, "error parsing time to die")?),
            time_to_eat: Duration::from_millis(string_to(tte, "error parsing time to sleep")?),
            time_to_sleep: Duration::from_millis(string_to(tts, "error parsing time to eat")?),
            must_eat: match must_eat {
                Some(value) => match string_to(value, "error parsing must_eat") {
                    Ok(must_eat) => Some(must_eat),
                    Err(err) => return Err(err),
                },
                _ => None,
            },
        })
    }
}

#[inline]
fn safe_print(action: &str, id: u64, start_time: Instant, print_mutex: &MPrint) {
    let _print_guard = print_mutex.lock().unwrap();
    println!("{} philo {} {action}", start_time.elapsed().as_millis(), id);
}

fn main() {
    let args: Vec<String> = args().collect();

    if !(5..=6).contains(&args.len()) {
        return println!("wrong nums of args {{num, die, eat, sleep}}");
    }

    let info = match Instructions::new(&args[1], &args[2], &args[3], &args[4], args.get(5)) {
        Ok(instructions) => instructions,
        Err(error) => return println!("{error}"),
    };

    dbg!(info);

    let (philo_maker, rx) = PhiloMaker::init(info);
    philo_maker.start_simulation();
    rx.recv().expect("Could not receive from channel.");
}
