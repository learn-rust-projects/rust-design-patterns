use super::has_mutable_component::HasMutableComponent;
use crate::components::cloud_component::CloudComponent;

pub trait PowerControl<C: CloudComponent>: HasMutableComponent<C> {
    fn enable(&mut self) {
        self.component().set_enabled(true);
    }
    fn disable(&mut self) {
        self.component().set_enabled(false);
    }
}
