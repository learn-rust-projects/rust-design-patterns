use std::rc::Rc;

pub trait Mediator {
    fn send_message(&self, from: &str, message: &str);
    fn register_user(&mut self, user: Rc<dyn User>);
}

pub trait User {
    fn name(&self) -> &str;
    fn receive(&self, from: &str, message: &str);
    fn send(&self, message: &str);
}
pub struct ChatRoomMediator {
    users: Vec<Rc<dyn User>>,
}

impl ChatRoomMediator {
    pub fn new() -> Self {
        ChatRoomMediator { users: vec![] }
    }
}

impl Mediator for ChatRoomMediator {
    fn send_message(&self, from: &str, message: &str) {
        for user in &self.users {
            if user.name() != from {
                user.receive(from, message);
            }
        }
    }

    fn register_user(&mut self, user: Rc<dyn User>) {
        self.users.push(user);
    }
}
