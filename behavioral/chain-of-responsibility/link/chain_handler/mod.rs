use crate::handler::Handler;

pub struct HandlerNode {
    handler: Box<dyn Handler>,
    next: Option<Box<HandlerNode>>,
}

impl HandlerNode {
    pub fn new(handler: Box<dyn Handler>) -> Self {
        Self {
            handler,
            next: None,
        }
    }

    /// Adds a handler to the end of the responsibility chain
    pub fn add(&mut self, handler: Box<dyn Handler>) {
        match &mut self.next {
            Some(next) => {
                next.add(handler);
            }
            None => {
                self.next = Some(Box::new(HandlerNode::new(handler)));
            }
        }
    }

    /// handle the request, if the current handler does not handle the request,
    /// pass it to the next handler
    pub fn handle(&self) {
        let handled = self.handler.do_handle();
        if !handled && let Some(next) = &self.next {
            next.handle();
        }
    }

    /// call all handlers on the chain, regardless of whether they are
    /// successful —— all handlers will be executed
    pub fn handle_without_check(&self) {
        // ignore whether the processing is successful
        self.handler.do_handle();
        if let Some(next) = &self.next {
            next.handle_without_check();
        }
    }
}
