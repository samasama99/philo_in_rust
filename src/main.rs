// #![allow(unused)]
use std::env::args;
use std::str::FromStr;
use std::sync::mpsc::{channel, Receiver, Sender};
use std::sync::{Arc, Mutex, MutexGuard};
use std::thread;
use std::time::{Duration, Instant};

type MInstant = Arc<Mutex<Instant>>;

type Fork = Arc<Mutex<u64>>;

struct Philo {
    id: u64,
    last_meal: MInstant,
    philos_meals: Arc<Mutex<u64>>,
    start_time: Instant,
    instructions: Instructions,
    print_mutex: Arc<Mutex<u8>>,
    left_fork: Fork,
    right_fork: Fork,
    meals: u64,
}

impl Philo {
    fn born(
        id: u64,
        left_fork: Fork,
        right_fork: Fork,
        print_mutex: Arc<Mutex<u8>>,
        philos_meals: Arc<Mutex<u64>>,
        start_time: Instant,
        instructions: Instructions,
        tx: Sender<()>,
    ) -> Self {
        let last_meal = Arc::new(Mutex::new(Instant::now()));
        let last_meal_checker = Arc::clone(&last_meal);
        let print_mutex_checker = Arc::clone(&print_mutex);
        let philos_meals_checker = Arc::clone(&philos_meals);
        let philo = Self {
            id,
            last_meal,
            philos_meals,
            start_time,
            instructions,
            print_mutex,
            left_fork,
            right_fork,
            meals: 0,
        };
        thread::spawn(move || loop {
            thread::sleep(Duration::from_millis(10));
            if instructions.must_eat.is_some()
                && *philos_meals_checker.lock().unwrap() == instructions.nums_of_philos
            {
                let _print_guard = print_mutex_checker.lock().unwrap();
                tx.send(()).expect("Could not send signal on channel.");
                loop {
                    thread::sleep(Duration::from_secs(u64::MAX));
                }
            };
            if last_meal_checker.lock().unwrap().elapsed() > instructions.time_to_die {
                let _print_guard = print_mutex_checker.lock().unwrap();
                println!("{} philo {id} is dead ", start_time.elapsed().as_millis());
                tx.send(()).expect("Could not send signal on channel.");
                thread::sleep(Duration::from_secs(u64::MAX));
            }
        });
        philo
    }

    fn eat(&mut self) {
        let guard1 = ForksPool::take_fork(&self.left_fork);
        safe_print("took a fork", self.id, self.start_time, &self.print_mutex);
        let guard2 = ForksPool::take_fork(&self.right_fork);
        safe_print("took a fork", self.id, self.start_time, &self.print_mutex);
        update_time(&self.last_meal);
        safe_print("is eating", self.id, self.start_time, &self.print_mutex);
        if let Some(must_eat) = self.instructions.must_eat {
            self.meals += 1;
            if must_eat == self.meals {
                *self.philos_meals.lock().unwrap() += 1;
            }
        };
        thread::sleep(self.instructions.time_to_eat);
        ForksPool::drop_fork(guard1);
        ForksPool::drop_fork(guard2);
    }

    fn sleep(&self) {
        safe_print("is sleeping", self.id, self.start_time, &self.print_mutex);
        thread::sleep(self.instructions.time_to_sleep);
    }

    fn think(self: &Self) {
        safe_print("is thinking", self.id, self.start_time, &self.print_mutex);
    }
}

struct PhiloMaker {
    nums_of_philos: u64,
    forks: ForksPool,
    print_mutex: Arc<Mutex<u8>>,
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
                print_mutex: Arc::new(Mutex::new(0)),
                philos_meals: Arc::new(Mutex::new(0)),
                initialised: true,
                philos_made: 0,
                instructions,
                tx,
            },
            rx,
        )
    }
    fn make(self: &mut Self) -> Philo {
        assert!(self.initialised);
        self.philos_made += 1;
        Philo::born(
            self.philos_made,
            self.forks.get(self.philos_made - 1),
            self.forks.get(self.philos_made % self.nums_of_philos),
            Arc::clone(&self.print_mutex),
            Arc::clone(&self.philos_meals),
            self.start_time,
            self.instructions,
            self.tx.clone(),
        )
    }
}

struct ForksPool {
    forks: Vec<Arc<Mutex<u64>>>,
}

impl ForksPool {
    fn init(nums_of_philos: u64) -> Self {
        let forks: Vec<Arc<Mutex<u64>>> = (0..nums_of_philos)
            .map(|_| Arc::new(Mutex::new(0)))
            .collect();
        ForksPool { forks }
    }
    fn get(&self, id: u64) -> Arc<Mutex<u64>> {
        Arc::clone(&self.forks[id as usize])
    }

    fn take_fork(fork: &Arc<Mutex<u64>>) -> MutexGuard<u64> {
        fork.lock().unwrap()
    }

    fn drop_fork(fork_guard: MutexGuard<u64>) {
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

fn safe_print(action: &str, id: u64, start_time: Instant, mutex: &Arc<Mutex<u8>>) {
    let _print_guard = mutex.lock().unwrap();
    println!(
        "{} philo {} {action}",
        start_time.elapsed().as_millis(),
        id + 1
    );
}

fn update_time(time: &MInstant) {
    *time.lock().unwrap() = Instant::now();
}

fn main() {
    let args: Vec<String> = args().collect();

    match args.len() {
        5 | 6 => (),
        _ => return println!("wrong nums of args {{num, die, eat, sleep}}"),
    };

    let info = match Instructions::new(&args[1], &args[2], &args[3], &args[4], args.get(5)) {
        Ok(instructions) => instructions,
        Err(error) => return println!("{error}"),
    };

    let (mut philo_maker, rx) = PhiloMaker::init(info);

    for _ in 0..info.nums_of_philos {
        let mut philo = philo_maker.make();
        thread::spawn(move || {
            thread::sleep(Duration::from_millis(5));
            loop {
                philo.eat();
                philo.sleep();
                philo.think();
            }
        });
    }

    rx.recv().expect("Could not receive from channel.");
}
