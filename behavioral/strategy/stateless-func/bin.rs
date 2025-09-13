// Define a strategy type: function pointer that takes a message
type LogStrategy = fn(&str);

// Log to console
fn console_log(message: &str) {
    println!("[Console] {message}");
}

// Log to a file
fn file_log(message: &str) {
    use std::{fs::OpenOptions, io::Write};

    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open("app.log")
        .expect("Failed to open log file");

    writeln!(file, "[File] {message}").expect("Failed to write log");
}

// Simulate logging over network
fn network_log(message: &str) {
    println!("[Network] Sending '{message}' to remote server...");
}

// Enum to identify strategy types
enum LogStrategyType {
    Console,
    File,
    Network,
}

// Factory to map enum to corresponding function
struct LogStrategyFactory;

impl LogStrategyFactory {
    pub fn get(strategy_type: LogStrategyType) -> LogStrategy {
        match strategy_type {
            LogStrategyType::Console => console_log,
            LogStrategyType::File => file_log,
            LogStrategyType::Network => network_log,
        }
    }
}

// Logger holds a strategy and calls it to log messages
struct Logger {
    log_strategy: LogStrategy,
}

impl Logger {
    pub fn new(strategy: LogStrategy) -> Self {
        Self {
            log_strategy: strategy,
        }
    }

    pub fn log(&self, message: &str) {
        (self.log_strategy)(message);
    }
}

fn main() {
    // Console strategy
    let logger = Logger::new(LogStrategyFactory::get(LogStrategyType::Console));
    logger.log("User logged in.");

    // File strategy
    let logger = Logger::new(LogStrategyFactory::get(LogStrategyType::File));
    logger.log("User clicked 'Settings'.");

    // Network strategy
    let logger = Logger::new(LogStrategyFactory::get(LogStrategyType::Network));
    logger.log("Error: failed to fetch data.");

    // Inline closure strategy (no factory needed)
    let logger = Logger::new(|msg| println!("[Temp] {msg}"));

    logger.log("Temporary debug log.");

    // ✅ Delete the log file (for demo or testing only)
    if let Err(e) = std::fs::remove_file("app.log") {
        eprintln!("Warning: Failed to remove log file: {e}");
    }
}
