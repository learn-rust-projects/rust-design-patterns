mod chat_room;
use std::{cell::RefCell, rc::Rc};

use chat_room::*;

fn main() {
    let mediator: Rc<RefCell<dyn Mediator>> = Rc::new(RefCell::new(ChatRoomMediator::new()));

    let alice = Rc::new(ChatUser::new("Alice", Rc::clone(&mediator)));
    let bob = Rc::new(ChatUser::new("Bob", Rc::clone(&mediator)));
    let charlie = Rc::new(ChatUser::new("Charlie", Rc::clone(&mediator)));

    mediator.borrow_mut().register_user(alice.clone());
    mediator.borrow_mut().register_user(bob.clone());
    mediator.borrow_mut().register_user(charlie.clone());

    let bob: Rc<dyn User> = bob as Rc<dyn User>;
    // Simulate user action
    bob.send("Hello everyone!");
}
