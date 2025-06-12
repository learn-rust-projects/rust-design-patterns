use crate::component::stream::Stream;

/// Decorator: UppercaseStream
///
/// This struct is another concrete Decorator that transforms the data of any Stream to uppercase.
/// Decorators can be combined (stacked) to provide multiple enhancements.
pub struct UppercaseStream<T: Stream> {
    inner: T, // The wrapped component
}

impl<T: Stream> UppercaseStream<T> {
    /// Wraps a Stream with uppercase transformation.
    pub fn new(inner: T) -> Self {
        Self { inner }
    }
}

impl<T: Stream> Stream for UppercaseStream<T> {
    fn read(&mut self) -> String {
        println!("Decorator 2 [UppercaseStream: buffering data...]");
        let data = self.inner.read();
        data.to_uppercase()
    }
}
