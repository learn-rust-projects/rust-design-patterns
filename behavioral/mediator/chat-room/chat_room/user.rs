use super::mediator::{Mediator, User};
use std::cell::RefCell;
use std::rc::Rc;

pub struct ChatUser {
    name: String,
    mediator: Rc<RefCell<dyn Mediator>>,
}

impl ChatUser {
    pub fn new(name: &str, mediator: Rc<RefCell<dyn Mediator>>) -> Self {
        ChatUser {
            name: name.to_string(),
            mediator,
        }
    }
}

impl User for ChatUser {
    fn name(&self) -> &str {
        &self.name
    }

    fn receive(&self, from: &str, message: &str) {
        println!(
            "{} received a message from {}: {}",
            self.name, from, message
        );
    }

    fn send(&self, message: &str) {
        println!("{} sends message: {}", self.name, message);
        self.mediator.borrow().send_message(&self.name, message);
    }
}
