use crate::components::cloud_component::CloudComponent;
use crate::features::has_mutable_component::HasMutableComponent;

pub struct CloudConfigurator<C: CloudComponent> {
    pub component: C,
}

impl<C: CloudComponent> CloudConfigurator<C> {
    pub fn new(component: C) -> Self {
        Self { component }
    }
}

impl<C: CloudComponent> HasMutableComponent<C> for CloudConfigurator<C> {
    fn component(&mut self) -> &mut C {
        &mut self.component
    }
}
