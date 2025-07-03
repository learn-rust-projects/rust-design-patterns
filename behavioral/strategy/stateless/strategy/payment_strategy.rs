pub trait PaymentStrategy {
    fn pay(&self, amount: f64);
}
