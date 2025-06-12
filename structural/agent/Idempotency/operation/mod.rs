pub trait Operation<T> {
    fn execute(&mut self, idempotency_key: Option<String>) -> T;
}
