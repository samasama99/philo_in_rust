use std::env::args;

use instructions::Instructions;
use philosopher_maker::PhilosopherMaker;

mod philosopher;
mod philosopher_maker;
mod forks_pool;
mod instructions;
mod utils;

fn main() {
    let args: Vec<String> = args().collect();

    if !(5..=6).contains(&args.len()) {
        return println!("wrong nums of args {{num, die, eat, sleep}}");
    }

    let info = match Instructions::new(
        &args[1],
        &args[2],
        &args[3],
        &args[4],
        args.get(5),
    ) {
        Ok(instructions) => instructions,
        Err(error) => return println!("{error}"),
    };

    #[cfg(debug_assertions)]{ dbg!(info); }

    let (philosopher_maker, rx) = PhilosopherMaker::init(info);
    philosopher_maker.start_simulation();
    rx.recv().expect("Could not receive from channel.");
}
