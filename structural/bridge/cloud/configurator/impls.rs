use crate::components::{
    cloud_component::CloudComponent, compute_instance::ComputeInstance, storage::Storage,
};
use crate::configurator::cloud_configurator::CloudConfigurator;
use crate::features::capacity_control::CapacityControl;
use crate::features::logging_decorator::LoggingDecorator;
use crate::features::power_control::PowerControl;
use crate::features::reset_control::ResetControl;

/// Bridge Pattern Implementation
///
/// - The `CloudConfigurator` serves as the abstraction, decoupling high-level features (traits)
///   from concrete cloud resources (CloudComponent implementors).
/// - Features like logging, power, and reset control are added via trait composition, not inheritance.
/// - This enables independent extension of both features and component types.
///
/// # Design Principles
/// - **Bridge Pattern:** Separates abstraction (features/traits) from implementation (components), allowing both to vary independently.
/// - **Composition over Inheritance:** Behaviors are composed at compile time via traits.
/// - **Open/Closed Principle:** Add new features or component types without modifying existing code.
impl<C: CloudComponent> PowerControl<C> for CloudConfigurator<C> {}
impl<C: CloudComponent> ResetControl<C> for CloudConfigurator<C> {}
impl<C: CloudComponent> LoggingDecorator for CloudConfigurator<C> {}

// Bridge specialization: concrete implementations for each component type.
impl CapacityControl<Storage> for CloudConfigurator<Storage> {
    fn get_capacity(&self) -> u32 {
        println!("Called get_capacity(), returning value: capacity_gb");
        self.component.get_capacity()
    }
}
impl CapacityControl<ComputeInstance> for CloudConfigurator<ComputeInstance> {
    fn get_capacity(&self) -> u32 {
        println!("Called get_capacity(), returning value: vcpu");
        self.component.get_capacity()
    }
}
