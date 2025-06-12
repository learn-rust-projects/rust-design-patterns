use crate::operation::Operation;

pub struct AddOne(pub i32);

impl Operation<i32> for AddOne {
    fn execute(&mut self, _idempotency_key: Option<String>) -> i32 {
        self.0 + 1
    }
}
