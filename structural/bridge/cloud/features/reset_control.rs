use super::has_mutable_component::HasMutableComponent;
use crate::components::cloud_component::CloudComponent;

pub trait ResetControl<C: CloudComponent>: HasMutableComponent<C> {
    fn reset(&mut self) {
        self.component().set_enabled(false);
        self.component().set_enabled(true);
        println!("Component has been reset!");
    }
}
