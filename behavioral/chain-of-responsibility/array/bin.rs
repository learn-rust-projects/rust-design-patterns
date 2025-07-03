// define the handler interface
pub trait Handler {
    fn handle(&self) -> bool;
}

// concrete handler A
pub struct HandlerA;

impl Handler for HandlerA {
    fn handle(&self) -> bool {
        let handled = false;
        println!("HandlerA executing... handled: {handled}");
        handled
    }
}

// concrete handler B
pub struct HandlerB;

impl Handler for HandlerB {
    fn handle(&self) -> bool {
        let handled = false;
        println!("HandlerB executing... handled: {handled}");
        handled
    }
}

// concrete handler C
pub struct HandlerC;

impl Handler for HandlerC {
    fn handle(&self) -> bool {
        let handled = true;
        println!("HandlerC executing... handled: {handled}");
        handled
    }
}

// responsibility chain manager
pub struct HandlerChain {
    handlers: Vec<Box<dyn Handler>>,
}
impl Default for HandlerChain {
    fn default() -> Self {
        Self::new()
    }
}

impl HandlerChain {
    pub fn new() -> Self {
        HandlerChain {
            handlers: Vec::new(),
        }
    }

    pub fn add_handler(&mut self, handler: Box<dyn Handler>) {
        self.handlers.push(handler);
    }

    pub fn handle(&self) {
        for handler in &self.handlers {
            let handled = handler.handle();
            if handled {
                break;
            }
        }
    }

    // call each handler, without checking if it is successful —— all handlers will be executed
    pub fn handle_without_check(&self) {
        for handler in &self.handlers {
            // ignore the return value, force all handlers to be executed
            handler.handle();
        }
    }
}

// usage example
fn main() {
    // early exit, only execute to C, because C's handle returns true
    let mut chain = HandlerChain::new();
    chain.add_handler(Box::new(HandlerA));
    chain.add_handler(Box::new(HandlerB));
    chain.add_handler(Box::new(HandlerC));
    chain.add_handler(Box::new(HandlerA));
    chain.handle();
    println!("===========");
    // not early exit, execute to A, because A's handle returns false
    let mut chain = HandlerChain::new();
    chain.add_handler(Box::new(HandlerA));
    chain.add_handler(Box::new(HandlerB));
    chain.add_handler(Box::new(HandlerA));
    chain.handle();
    println!("===========");
    // call each handler, without checking if it is successful —— all handlers will be executed
    let mut chain = HandlerChain::new();
    chain.add_handler(Box::new(HandlerA));
    chain.add_handler(Box::new(HandlerB));
    chain.add_handler(Box::new(HandlerC));
    chain.add_handler(Box::new(HandlerA));
    chain.handle_without_check();
}
