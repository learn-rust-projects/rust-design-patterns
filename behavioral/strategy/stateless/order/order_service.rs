use crate::strategy::PaymentStrategy;

pub struct OrderService {
    strategy: Box<dyn PaymentStrategy>,
}

pub struct OrderStaticService<T: PaymentStrategy> {
    strategy: T,
}

impl OrderService {
    pub fn new(strategy: Box<dyn PaymentStrategy>) -> Self {
        Self { strategy }
    }

    pub fn checkout(&self, amount: f64) {
        println!("Starting checkout...");
        self.strategy.pay(amount);
        println!("Payment completed.");
    }
}

impl<T: PaymentStrategy> OrderStaticService<T> {
    pub fn new(strategy: T) -> Self {
        Self { strategy }
    }

    pub fn checkout(&self, amount: f64) {
        println!("Starting checkout...");
        self.strategy.pay(amount);
        println!("Payment completed.");
    }
}
