use std::str::FromStr;
use std::sync::{Arc, Mutex};
use std::time::Instant;

pub type MInstant = Arc<Mutex<Instant>>;
pub type MPrint = Arc<Mutex<()>>;
pub type Fork = Arc<Mutex<()>>;

pub fn string_to<T: FromStr>(target: &str, err: &str) -> Result<T, String> {
    match target.parse::<T>() {
        Ok(value) => Ok(value),
        _ => Err(err.to_string()),
    }
}

#[inline]
pub fn safe_print(action: &str, id: u64, start_time: Instant, print_mutex: &MPrint) {
    let _print_guard = print_mutex.lock().expect("safe_print failed");
    println!("{} philosopher {} {action}", start_time.elapsed().as_millis(), id);
}

