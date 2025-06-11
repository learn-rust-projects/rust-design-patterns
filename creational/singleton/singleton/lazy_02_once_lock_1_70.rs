// Rust 1.70+  thread-safe lazy initialization
use std::sync::{Mutex, OnceLock};

fn array() -> &'static Mutex<Vec<u8>> {
    static ARRAY: OnceLock<Mutex<Vec<u8>>> = OnceLock::new();
    ARRAY.get_or_init(|| Mutex::new(vec![]))
}

fn do_a_call() {
    array().lock().unwrap().push(1);
}

fn main() {
    do_a_call();
    do_a_call();
    do_a_call();

    println!("called {}", array().lock().unwrap().len());
}
