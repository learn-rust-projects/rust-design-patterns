mod handler_a;
mod handler_b;

pub use handler_a::HandlerA;
pub use handler_b::HandlerB;

pub trait Handler {
    /// 处理逻辑，返回是否已处理
    fn do_handle(&self) -> bool;
}
