// Custom thread-safe lazy loading cell
use std::cell::UnsafeCell;
use std::sync::Once;

// UnsafeCell<T> is !Sync, manual Sync implementation requires thread safety guarantee
pub struct MyOnceCell<T> {
    once: Once,
    value: UnsafeCell<Option<T>>,
}

impl<T> MyOnceCell<T> {
    pub const fn new() -> Self {
        MyOnceCell {
            once: Once::new(),
            value: UnsafeCell::new(None),
        }
    }

    // Get reference to initialized value, or initialize it
    pub fn get_or_init<F>(&self, f: F) -> &T
    where
        F: FnOnce() -> T,
    {
        // Once ensures closure is called only once
        self.once.call_once(|| {
            let value = f();
            unsafe {
                *self.value.get() = Some(value);
            }
        });

        // Initialization complete, safe to read
        unsafe {
            (*self.value.get())
                .as_ref()
                .expect("value must be initialized")
        }
    }

    // Try to get reference to initialized value
    pub fn get(&self) -> Option<&T> {
        unsafe { (*self.value.get()).as_ref() }
    }
}

impl<T> Default for MyOnceCell<T> {
    fn default() -> Self {
        Self::new()
    }
}

// Safety: MyOnceCell is thread-safe because:
// 1. Once ensures initialization happens only once
// 2. After initialization, the value is immutable
unsafe impl<T: Send + Sync> Sync for MyOnceCell<T> {}

fn main() {
    static CELL: MyOnceCell<String> = MyOnceCell::new();

    let val = CELL.get_or_init(|| "Hello, singleton!".to_string());
    println!("{val}");

    // Access again, returns reference directly without re-initialization
    let val2 = CELL.get_or_init(|| unreachable!("won't be called"));
    println!("{val2}");
}
