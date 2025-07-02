/// The `LogProcessor` trait defines the skeleton of a log processing algorithm.
/// It uses the Template Method pattern where the `process()` method defines
/// the fixed flow, and each step can be optionally overridden by implementors.
trait LogProcessor {
    /// Step 1: Read the log file (can be overridden).
    fn read(&self) {
        println!("[Default] Reading logs...");
    }

    /// Step 2: Parse the log data (can be overridden).
    fn parse(&self) {
        println!("[Default] Parsing logs...");
    }

    /// Step 3: Save the results (can be overridden).
    fn save(&self) {
        println!("[Default] Saving results to the database...");
    }

    /// The Template Method: defines the fixed algorithm structure.
    /// This method should not be overridden.
    fn process(&self) {
        println!("== Begin Log Processing ==");
        self.read(); // Hook method: optionally overridden
        self.parse(); // Hook method: optionally overridden
        self.save(); // Hook method: optionally overridden
        println!("== Log Processing Finished ==\n");
    }
}

/// A concrete implementation for processing Apache logs.
struct ApacheLogProcessor;

impl LogProcessor for ApacheLogProcessor {
    fn read(&self) {
        println!("Reading logs from /var/log/apache.log...");
    }

    fn parse(&self) {
        println!("Parsing logs using Apache log format...");
    }

    // Uses default `save()` implementation
}

/// A concrete implementation for processing Nginx logs.
struct NginxLogProcessor;

impl LogProcessor for NginxLogProcessor {
    fn read(&self) {
        println!("Reading logs from /var/log/nginx.log...");
    }

    fn parse(&self) {
        println!("Parsing logs using Nginx log format...");
    }

    fn save(&self) {
        println!("Saving parsed data into PostgreSQL database...");
    }
}

/// The client code demonstrating the use of the Template Method.
fn main() {
    let apache = ApacheLogProcessor;
    let nginx = NginxLogProcessor;

    apache.process(); // Runs Apache log processing flow
    nginx.process(); // Runs Nginx log processing flow
}
