// #![allow(unused)]
use std::env::args;
use std::str::FromStr;
use std::sync::mpsc::channel;
use std::sync::{Arc, Mutex, MutexGuard};
use std::thread;
use std::time::{Duration, Instant};

type MInstant = Arc<Mutex<Instant>>;

struct ForksPool {
    forks: Vec<Arc<Mutex<usize>>>,
}

/// hello
impl ForksPool {
    fn init(nums_of_philos: usize) -> Self {
        let forks: Vec<Arc<Mutex<usize>>> = (0..nums_of_philos)
            .map(|_| Arc::new(Mutex::new(0)))
            .collect();
        ForksPool { forks }
    }
    fn get(&self, id: usize) -> Arc<Mutex<usize>> {
        Arc::clone(&self.forks[id])
    }

    fn take_fork(fork: &Arc<Mutex<usize>>) -> MutexGuard<usize> {
        fork.lock().unwrap()
    }

    fn drop_fork(fork_guard: MutexGuard<usize>) {
        drop(fork_guard)
    }
}

#[derive(Debug, Default, Clone, Copy)]
struct Instructions {
    nums_of_philos: usize,
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
            nums_of_philos: string_to(nums_of_philos, "error parsing nums of philos")?,
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

fn safe_print(action: &str, id: usize, start_time: Instant, mutex: &Arc<Mutex<usize>>) {
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

    let instructions = match Instructions::new(&args[1], &args[2], &args[3], &args[4], args.get(5))
    {
        Ok(instructions) => instructions,
        Err(error) => return println!("{error}"),
    };

    let (tx, rx) = channel();
    let forks = ForksPool::init(instructions.nums_of_philos);
    let p_mutex = Arc::new(Mutex::new(0));
    let must_eat_rec = Arc::new(Mutex::new(0));
    let start_time = Instant::now();
    (0..instructions.nums_of_philos).for_each(|id| {
        let fork1 = forks.get(id);
        let fork2 = forks.get((id + 1) % instructions.nums_of_philos);
        let p_mutex = Arc::clone(&p_mutex);
        let must_eat_rec = Arc::clone(&must_eat_rec);
        let tx = tx.clone();
        thread::spawn(move || {
            if id % 2 == 0 {
                thread::sleep(Duration::from_millis(5));
            }

            let last_meal = Arc::new(Mutex::new(Instant::now()));
            let last_meal_checker = Arc::clone(&last_meal);
            let p_mutex_checker = Arc::clone(&p_mutex);
            let must_eat_rec_checker = Arc::clone(&must_eat_rec);
            let mut philo_eat = 0;

            thread::spawn(move || loop {
                thread::sleep(Duration::from_millis(5));
                if instructions.must_eat.is_some()
                    && *must_eat_rec_checker.lock().unwrap() == instructions.nums_of_philos
                {
                    let _print_guard = p_mutex_checker.lock().unwrap();
                    tx.send(()).expect("Could not send signal on channel.");
                    loop {
                        thread::sleep(Duration::from_secs(u64::MAX));
                    }
                };
                if last_meal_checker.lock().unwrap().elapsed() > instructions.time_to_die {
                    let _print_guard = p_mutex_checker.lock().unwrap();
                    println!("{} philo {id} is dead ", start_time.elapsed().as_millis());
                    tx.send(()).expect("Could not send signal on channel.");
                    loop {
                        thread::sleep(Duration::from_secs(u64::MAX));
                    }
                }
            });

            loop {
                let guard2 = ForksPool::take_fork(&fork2);
                safe_print("took a right fork", id, start_time, &p_mutex);
                let guard1 = ForksPool::take_fork(&fork1);
                safe_print("took a left fork", id, start_time, &p_mutex);
                update_time(&last_meal);
                safe_print("is eating", id, start_time, &p_mutex);
                if let Some(must_eat) = instructions.must_eat {
                    philo_eat += 1;
                    if must_eat == philo_eat {
                        *must_eat_rec.lock().unwrap() += 1;
                    }
                };
                thread::sleep(instructions.time_to_eat);
                ForksPool::drop_fork(guard1);
                ForksPool::drop_fork(guard2);
                safe_print("is sleeping", id, start_time, &p_mutex);
                thread::sleep(instructions.time_to_sleep);
                safe_print("is thinking", id, start_time, &p_mutex);
            }
        });
    });

    rx.recv().expect("Could not receive from channel.");
}
