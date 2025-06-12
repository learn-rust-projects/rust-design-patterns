use super::has_mutable_component::HasMutableComponent;
use crate::components::cloud_component::CloudComponent;

pub trait CapacityControl<C: CloudComponent>: HasMutableComponent<C> {
    fn get_capacity(&self) -> u32;
    fn increase_capacity(&mut self, amount: u32) {
        let new_capacity = self.get_capacity() + amount;
        self.component().set_capacity(new_capacity);
    }
    fn decrease_capacity(&mut self, amount: u32) {
        let current = self.get_capacity();
        let new_capacity = current.saturating_sub(amount);
        self.component().set_capacity(new_capacity);
    }
}
