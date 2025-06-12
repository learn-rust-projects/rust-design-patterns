use crate::component::stream::Stream;

/// Decorator: BufferedStream
///
/// This struct is a concrete Decorator that adds "buffering" behavior to any Stream.
/// It holds another Stream (the Component) and enhances its functionality.
///
/// # Design Pattern Principle:
/// - Decorators wrap a component, implementing the same trait, and can be stacked dynamically.
/// - This enables flexible, runtime behavior extension without subclassing.
///
/// # Open/Closed Principle:
/// - Functionality can be extended (via composition) without modifying the original Stream code.
pub struct BufferedStream<T: Stream> {
    inner: T, // The wrapped component
}

impl<T: Stream> BufferedStream<T> {
    /// Wraps a Stream with buffering capability.
    pub fn new(inner: T) -> Self {
        Self { inner }
    }
}

impl<T: Stream> Stream for BufferedStream<T> {
    fn read(&mut self) -> String {
        // Example buffered behavior (could be real buffering in production)
        println!("Decorator 1 [BufferedStream: buffering data...]");
        self.inner.read()
    }
}
