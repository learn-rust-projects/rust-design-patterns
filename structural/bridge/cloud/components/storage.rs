use super::cloud_component::CloudComponent;

pub struct Storage {
    pub enabled: bool,
    pub capacity_gb: u32,
}

impl CloudComponent for Storage {
    fn set_enabled(&mut self, enabled: bool) {
        self.enabled = enabled;
        println!("Storage enabled: {enabled}");
    }
    fn set_capacity(&mut self, capacity: u32) {
        self.capacity_gb = capacity;
        println!("Storage capacity set to: {capacity} GB");
    }
    fn get_capacity(&self) -> u32 {
        self.capacity_gb
    }
}
