pub trait LoggingDecorator {
    fn log_action(&self, action: &str) {
        println!("[LOG] Action: {action}");
    }
}
