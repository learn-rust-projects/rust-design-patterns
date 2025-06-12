pub trait CloudComponent {
    fn set_enabled(&mut self, enabled: bool);
    fn set_capacity(&mut self, capacity: u32);
    fn get_capacity(&self) -> u32;
}
