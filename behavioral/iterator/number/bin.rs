/// The actual iterator that generates numbers from 1 up to `max`
pub struct Counter {
    current: usize,
    max: usize,
}

impl Counter {
    /// Create a new counter that counts up to `max`
    fn new(max: usize) -> Self {
        Counter { current: 0, max }
    }
}

impl Iterator for Counter {
    type Item = usize;

    /// Returns the next number in the sequence, or None if complete
    fn next(&mut self) -> Option<Self::Item> {
        if self.current < self.max {
            self.current += 1;
            Some(self.current)
        } else {
            None
        }
    }
}

/// Container that holds configuration for producing counters
pub struct CounterCollection {
    max: usize,
}

impl CounterCollection {
    /// Create a new container with a given upper bound
    pub fn new(max: usize) -> Self {
        Self { max }
    }

    /// Return an iterator (a fresh Counter) for the collection
    pub fn iter(&self) -> Counter {
        Counter::new(self.max)
    }
}

fn main() {
    // Create a counter collection
    let collection = CounterCollection::new(5);

    // Iterate over it using .iter()
    for value in collection.iter() {
        println!("Got: {value}");
    }

    // Reuse .iter() to get a new iterator
    let mut iter = collection.iter();
    assert_eq!(iter.next(), Some(1));
    assert_eq!(iter.next(), Some(2));
    assert_eq!(iter.next(), Some(3));
    assert_eq!(iter.next(), Some(4));
    assert_eq!(iter.next(), Some(5));
    assert_eq!(iter.next(), None);
}
