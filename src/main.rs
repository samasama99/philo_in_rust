// #![allow(unused)]
use std::env::args;
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

#[derive(Debug, Default)]
struct Instructions {
    time_to_die: Duration,
    time_to_eat: Duration,
    time_to_sleep: Duration,
}

impl Instructions {
    fn new(ttd: &str, tte: &str, tts: &str) -> Result<Self, String> {
        let ttd: u64 = match ttd.parse() {
            Ok(ttd) => ttd,
            _ => return Err("error parsing time to die to A integer".to_string()),
        };
        let tts: u64 = match tts.parse() {
            Ok(tts) => tts,
            _ => return Err("error parsing time to sleep to A integer".to_string()),
        };
        let tte: u64 = match tte.parse() {
            Ok(tte) => tte,
            _ => return Err("error parsing time to eat to A integer".to_string()),
        };
        Ok(Instructions {
            time_to_die: Duration::from_millis(ttd),
            time_to_eat: Duration::from_millis(tte),
            time_to_sleep: Duration::from_millis(tts),
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
    let args = args().collect::<Vec<String>>();
    if args.len() != 5 {
        return println!("wrong nums of args {{num, die, eat, sleep}}");
    };
    let nums_of_philos = match args[1].parse() {
        Ok(nums_of_philos) => nums_of_philos,
        _ => return println!("error parsing nums of philos to A integer"),
    };

    let instructions = match Instructions::new(&args[2], &args[3], &args[4]) {
        Ok(instructions) => instructions,
        Err(error) => return println!("{error}"),
    };

    let (tx, rx) = channel();
    let forks = ForksPool::init(nums_of_philos);
    let p_mutex = Arc::new(Mutex::new(0));
    let start_time = Instant::now();
    (0..nums_of_philos).for_each(|id| {
        let fork1 = forks.get(id);
        let fork2 = forks.get((id + 1) % nums_of_philos);
        let p_mutex_checker = Arc::clone(&p_mutex);
        let p_mutex = Arc::clone(&p_mutex);
        let tx = tx.clone();
        thread::spawn(move || {
            let last_meal = Arc::new(Mutex::new(Instant::now()));
            let last_meal_checker = Arc::clone(&last_meal);
            if id % 2 == 0 {
                thread::sleep(Duration::from_millis(5));
            }
            thread::spawn(move || loop {
                thread::sleep(Duration::from_millis(5));
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
                let guard1 = ForksPool::take_fork(&fork1);
                safe_print("took a left fork", id, start_time, &p_mutex);
                let guard2 = ForksPool::take_fork(&fork2);
                safe_print("took a right fork", id, start_time, &p_mutex);
                update_time(&last_meal);
                safe_print("is eating", id, start_time, &p_mutex);
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
