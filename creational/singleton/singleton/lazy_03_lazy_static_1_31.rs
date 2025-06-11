// For Rust 1.31+, macro-based implementation (gradually being replaced by once_cell)
// You get a `&'static Logger`, which is a shared immutable reference, wrapped in `Mutex<Config>`, making it **mutable + thread-safe**

use lazy_static::lazy_static;
use std::sync::Mutex;

struct Config {
    value: i32,
}

struct Logger {
    config: Mutex<Config>,
}

impl Logger {
    fn new() -> Self {
        Self {
            config: Mutex::new(Config { value: 0 }),
        }
    }

    fn increment(&self) -> i32 {
        let mut config = self.config.lock().unwrap();
        config.value += 1;
        config.value
    }
}

lazy_static! {
    static ref LOGGER: Logger = Logger::new();
}

fn main() {
    let val1 = LOGGER.increment();
    println!("Current value: {val1}");

    let val2 = LOGGER.increment();
    println!("Current value: {val2}");
}
