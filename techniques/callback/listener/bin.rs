use std::rc::Rc;

/// Define a trait for click event listeners, analogous to Java's
/// OnClickListener interface
pub trait ClickListener {
    fn on_click(&self);
}

/// Button struct that holds a list of listeners
pub struct Button {
    listeners: Vec<Rc<dyn ClickListener>>,
}
impl Default for Button {
    fn default() -> Self {
        Self::new()
    }
}

impl Button {
    pub fn new() -> Self {
        Self {
            listeners: Vec::new(),
        }
    }

    /// Register a listener
    pub fn add_click_listener(&mut self, listener: Rc<dyn ClickListener>) {
        self.listeners.push(listener);
    }

    /// Simulate clicking the button, triggering callbacks
    pub fn click(&self) {
        println!("[Button] clicked! Dispatching events...");
        for listener in &self.listeners {
            listener.on_click();
        }
    }
}

/// Example listener A for business logic
struct Logger;
impl ClickListener for Logger {
    fn on_click(&self) {
        println!("[Logger] Button was clicked. Logging event.");
    }
}

/// Example listener B for business logic
struct Analytics;
impl ClickListener for Analytics {
    fn on_click(&self) {
        println!("[Analytics] Tracking button click.");
    }
}

fn main() {
    let mut button = Button::new();

    // Add two listeners
    button.add_click_listener(Rc::new(Logger));
    button.add_click_listener(Rc::new(Analytics));

    // Simulate button click
    button.click();
}
