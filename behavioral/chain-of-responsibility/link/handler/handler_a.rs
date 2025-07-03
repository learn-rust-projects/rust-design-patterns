use super::Handler;
pub struct HandlerA;

impl Handler for HandlerA {
    fn do_handle(&self) -> bool {
        println!("handle A");
        // return false to indicate that the processing is not completed, continue to pass
        false
    }
}
