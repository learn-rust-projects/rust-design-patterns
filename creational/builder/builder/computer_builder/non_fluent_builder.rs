// non_fluent_builder.rs
// Non-Fluent Interface

use super::computer::Computer;
// #[derive(Default)]
pub struct NonFluentComputerBuilder {
    cpu: Option<String>,
    memory: Option<u32>,
    storage: Option<u32>,
    gpu: Option<String>,
}

impl Default for NonFluentComputerBuilder {
    fn default() -> Self {
        Self {
            cpu: Some("i5".to_string()),
            memory: Some(8),
            storage: Some(256),
            gpu: None,
        }
    }
}
impl NonFluentComputerBuilder {
    pub fn set_cpu(&mut self, cpu: &str) {
        self.cpu = Some(cpu.to_string());
    }

    pub fn set_memory(&mut self, gb: u32) {
        self.memory = Some(gb);
    }

    pub fn set_storage(&mut self, gb: u32) {
        self.storage = Some(gb);
    }

    pub fn set_gpu(&mut self, gpu: &str) {
        self.gpu = Some(gpu.to_string());
    }

    pub fn build(self) -> Computer {
        Computer {
            cpu: self.cpu.expect("Please, set a cpu"),
            memory: self.memory.expect("Please, set a memory"),
            storage: self.storage.expect("Please, set a storage"),
            gpu: self.gpu,
        }
    }
}
