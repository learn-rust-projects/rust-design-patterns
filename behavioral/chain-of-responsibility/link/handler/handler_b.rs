use super::Handler;

pub struct HandlerB;

impl Handler for HandlerB {
    fn do_handle(&self) -> bool {
        println!("handle B");
        false
    }
}
