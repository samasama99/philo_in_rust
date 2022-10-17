use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

// type fork_mutex = Arc<Mutex<_>>

// struct Fork {
//     fork :fork_mutex,
// };

// impl Fork {
//
// }

// #[derive(Debug, Default)]
// struct Instructions {
//     time_to_sleep: Duration,
//     time_to_eat: Duration,
//     time_to_die: Duration,
// }

// #[derive(Debug, Default)]
// struct Philo {
//     id: u32,
// }

fn main() {
    // let fork = Arc::new(Mutex::new(0));
    // let fork2 = Arc::new(Mutex::new(0));

    // let take_fork = Arc::clone(&fork);
    // let take_fork_2 = Arc::clone(&fork2);
    // let h1 = thread::spawn(move || {
    //     let mut a = take_fork.lock().unwrap();
    //     println!("philo1 took fork 1");
    //     let mut b = take_fork_2.lock().unwrap();
    //     println!("philo1 took fork 2");
    //     thread::sleep(Duration::from_secs(1));
    //     *a += 1;
    //     *b += 1;
    // });
    // let take_fork = Arc::clone(&fork);
    // let take_fork_2 = Arc::clone(&fork2);
    // let h2 = thread::spawn(move || {
    //     let mut a = take_fork.lock().unwrap();
    //     println!("philo2 took fork 1");
    //     let mut b = take_fork_2.lock().unwrap();
    //     println!("philo2 took fork 2");
    //     thread::sleep(Duration::from_secs(1));
    //     *a += 1;
    //     *b += 1;
    // });

    // h1.join().unwrap();
    // h2.join().unwrap();
    // println!("{} {}", fork.lock().unwrap(), fork2.lock().unwrap());
    let fork = Arc::new(Mutex::new(0));
    // let fork2 = Arc::new(Mutex::new(0));
    println!("starting the simulation");
    let mut handlers = Vec::new();
    (0..5).for_each(|id| {
        let fork_mutex = Arc::clone(&fork);
        handlers.push(thread::spawn(move || loop {
            let mut a = fork_mutex.lock().unwrap();
            println!("philo {id} took a fork");
            thread::sleep(Duration::from_secs(1));
            *a = 0;
            drop(a);
            thread::sleep(Duration::from_secs(1));
        }))
    });

    println!("joining");
    for handler in handlers {
        handler.join().unwrap();
    }
    println!("end of the simulation");
    println!("{}", fork.lock().unwrap());
    // // println!("{}", fork2.lock().unwrap());
    // println!("joined");

    // let x = Arc::new(Mutex::new(0));
    //  println!("starting the simulation");
    //  let mut handlers = Vec::new();
    //  (0..100).for_each(|id| {
    //      // println!("creating thread {id}");
    //      let x_mutex = Arc::clone(&x);
    //      handlers.push(thread::spawn(move || {
    //          for _ in 0..10000 {
    //              let mut x_data = x_mutex.lock().unwrap();
    //              *x_data += 1;
    //              println!("changing x in {}", id);
    //              thread::sleep(Duration::from_secs(1));
    //              drop(x_data);
    //              thread::sleep(Duration::from_secs(1));
    //          }
    //      }))
    //  });

    //  println!("joining");
    //  for handler in handlers {
    //      handler.join().unwrap();
    //  }
    //  println!("end of the simulation");
    //  println!("{}", x.lock().unwrap());
    //  println!("joined");
}
