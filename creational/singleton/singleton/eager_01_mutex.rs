use std::sync::Mutex;

/// Eager singleton, initialized when program starts
static INSTANCE: Singleton = Singleton::new();

pub struct Singleton {
    data: Mutex<Vec<u8>>,
}

impl Singleton {
    /// const constructor (eager singleton requires initialization at compile
    /// time)
    pub const fn new() -> Self {
        Self {
            data: Mutex::new(Vec::new()),
        }
    }

    /// Get reference to singleton
    pub fn get_data(&self) -> std::sync::MutexGuard<'_, Vec<u8>> {
        self.data.lock().unwrap()
    }

    /// Wrapper method: push data
    pub fn push(&self, val: u8) {
        self.data.lock().unwrap().push(val);
    }

    /// Wrapper method: get length
    pub fn len(&self) -> usize {
        self.data.lock().unwrap().len()
    }

    /// Check if empty
    pub fn is_empty(&self) -> bool {
        let data = self.data.lock().unwrap();
        data.is_empty()
    }
}

impl Default for Singleton {
    fn default() -> Self {
        Self::new()
    }
}

fn main() {
    // Use directly, no initialization needed
    INSTANCE.push(10);
    INSTANCE.push(20);

    let data = INSTANCE.get_data();
    println!("Length: {}, Data: {:?}", data.len(), data);
}
