use super::cloud_component::CloudComponent;

pub struct ComputeInstance {
    pub enabled: bool,
    pub vcpus: u32,
}

impl CloudComponent for ComputeInstance {
    fn set_enabled(&mut self, enabled: bool) {
        self.enabled = enabled;
        println!("Compute instance enabled: {enabled}");
    }
    fn set_capacity(&mut self, vcpus: u32) {
        self.vcpus = vcpus;
        println!("Compute instance vCPUs set to: {vcpus}");
    }
    fn get_capacity(&self) -> u32 {
        self.vcpus
    }
}
