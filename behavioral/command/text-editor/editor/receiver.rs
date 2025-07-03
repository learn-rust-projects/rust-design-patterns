#[derive(Clone)]
pub struct Editor {
    buffer: String,
}

impl Editor {
    pub fn new() -> Self {
        Self {
            buffer: String::new(),
        }
    }

    pub fn insert(&mut self, text: &str) {
        self.buffer.push_str(text);
    }

    pub fn backspace(&mut self, count: usize) {
        for _ in 0..count {
            self.buffer.pop();
        }
    }

    pub fn content(&self) -> &str {
        &self.buffer
    }
}
