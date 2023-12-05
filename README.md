![Header](./Philosophers.png)

# Philosopher

This project simulates the dining philosophers problem using threads and mutexes in Rust. The goal is to implement a solution that ensures the philosophers can eat without encountering issues like deadlocks or starvation.

## Table of Contents
- [Installation](#installation)
- [Usage](#usage)
- [Program Arguments](#program-arguments)
- [Example Command](#example-command)
- [Logging](#logging)
- [Contributing](#contributing)
- [License](#license)

## Installation

Make sure you have Rust and Cargo installed.

## Usage
To run the program directly without building, use the following command:

```shell
cargo run --release -- [number_of_philosophers] [time_to_die] [time_to_eat] [time_to_sleep] [number_of_times_each_philosopher_must_eat]
```
## Program Arguments
- number_of_philosophers: The number of philosophers and forks. Each philosopher will have a fork on their left and right sides.
- time_to_die: The time (in milliseconds) a philosopher can go without eating before they die.
- time_to_eat: The time (in milliseconds) it takes for a philosopher to eat.
- time_to_sleep: The time (in milliseconds) a philosopher spends sleeping.
- number_of_times_each_philosopher_must_eat (optional): The number of times each philosopher must eat before the simulation stops. If not specified, the simulation stops when a philosopher dies.
## Here's an example command to run the Rust program:

```shell
cargo run --release -- 5 800 200 200
```
This will simulate 5 philosophers, with a time to die of 800 milliseconds, time to eat of 200 milliseconds, and time to sleep of 200 milliseconds. The simulation will run until a philosopher dies.

## Logging
The program logs the state changes of the philosophers. Each state change is displayed with a timestamp in milliseconds and the philosopher number. The possible states are:

* has taken a fork
* is eating
* is sleeping
* is thinking
* died
## Contributing
Contributions to this project are welcome. If you find any issues or have suggestions for improvements, please open an issue or submit a pull request.

## License
This project is licensed under the MIT License.