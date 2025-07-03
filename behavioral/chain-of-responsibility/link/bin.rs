use chain_link::{HandlerA, HandlerB, HandlerNode};
fn main() {
    let mut chain = HandlerNode::new(Box::new(HandlerA));
    chain.add(Box::new(HandlerB));
    chain.add(Box::new(HandlerA)); // add another A

    chain.handle();
}
