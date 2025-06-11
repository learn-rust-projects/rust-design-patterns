mod encouragement;
// Introduction to the Simple Factory Pattern
// The Simple Factory Pattern refers to using a function (the factory) that decides which specific object to instantiate based on the parameters passed in, and then returns it.
// This pattern encapsulates the object creation logic in one place, so the caller does not need to know how the object is created—they just use it.

// Advantages: Decouples object creation from usage, centralizes logic, and is easy to maintain.
// Disadvantages: As the number of product types increases, the factory function can become quite large.
fn main() {
    let progresses = [80, 90, 100];
    for &progress in &progresses {
        let encourager = encouragement::EncouragementFactory::get_encourager(progress);
        println!(
            "Progress: {}, Message: {}",
            progress,
            encourager.encourage()
        );
    }
}
