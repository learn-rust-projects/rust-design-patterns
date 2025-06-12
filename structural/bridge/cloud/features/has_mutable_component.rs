use crate::components::cloud_component::CloudComponent;

pub trait HasMutableComponent<C: CloudComponent> {
    fn component(&mut self) -> &mut C;
}
