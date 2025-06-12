/// The core trait: Stream
/// This trait represents the abstract interface for all stream types.
/// In the Decorator Pattern, this is the Component abstraction.
/// All concrete streams and decorators will implement this trait.
pub trait Stream {
    /// Reads data from the stream and returns it as a String
    fn read(&mut self) -> String;
}

/// Concrete Component: FileStream
/// This struct simulates a file stream providing raw data.
/// In the Decorator Pattern, this is the ConcreteComponent.
pub struct FileStream;

impl Stream for FileStream {
    fn read(&mut self) -> String {
        // Simulate reading from a file
        "Hello, decorator pattern!".to_string()
    }
}
