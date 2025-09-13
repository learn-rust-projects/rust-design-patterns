fn main() {
    // Iterate and use all registered strategies
    for (name, constructor) in autostrategy::strategies::STRATEGIES.lock().unwrap().iter() {
        let strategy = constructor();
        println!("Using {}: {}", name, strategy.pay(100.0));
    }
}
