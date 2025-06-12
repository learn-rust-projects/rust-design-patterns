mod components;
mod configurator;
mod features;

use components::compute_instance::ComputeInstance;
use components::storage::Storage;
use configurator::cloud_configurator::CloudConfigurator;
use features::capacity_control::CapacityControl;
use features::logging_decorator::LoggingDecorator;
use features::power_control::PowerControl;
use features::reset_control::ResetControl;

fn main() {
    let storage = Storage {
        enabled: false,
        capacity_gb: 500,
    };
    let mut storage_manager = CloudConfigurator::new(storage);

    storage_manager.log_action("Enable storage");
    storage_manager.enable();
    storage_manager.increase_capacity(100);
    storage_manager.decrease_capacity(50);
    storage_manager.get_capacity();
    storage_manager.reset();

    let compute = ComputeInstance {
        enabled: true,
        vcpus: 4,
    };
    let mut compute_manager = CloudConfigurator::new(compute);

    compute_manager.log_action("Disable compute instance");
    compute_manager.disable();
    compute_manager.increase_capacity(2);
    compute_manager.decrease_capacity(1);
    compute_manager.get_capacity();
    compute_manager.reset();
}
