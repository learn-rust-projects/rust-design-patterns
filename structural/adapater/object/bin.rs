// === Original class (from legacy system) ===
struct Adaptee;

impl Adaptee {
    pub fn fa(&self) {
        println!("Calling fa() from Adaptee");
    }

    pub fn _fb(&self) {
        println!("Calling fb() from Adaptee");
    }
}

// === Target interface required by the new system ===
trait Target {
    fn f1(&self); // Corresponds to Adaptee::fa
    fn f2(&self); // New functionality
}

// === Adapter: uses composition to bridge Adaptee with Target interface ===
struct Adapter {
    adaptee: Adaptee,
}

impl Adapter {
    pub fn new(adaptee: Adaptee) -> Self {
        Self { adaptee }
    }
}

// Implement the Target trait for the Adapter
impl Target for Adapter {
    fn f1(&self) {
        self.adaptee.fa(); // Delegate to the Adaptee's fa method
    }

    fn f2(&self) {
        println!("New implementation of f2()");
    }
}

// === Entry point for testing ===
fn main() {
    let adaptee = Adaptee;
    let adapter = Adapter::new(adaptee);

    adapter.f1(); // Calls the legacy method via adapter
    adapter.f2(); // Calls the newly defined method
}
