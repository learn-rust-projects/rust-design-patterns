// fluent_builder.rs
// Fluent Interface

use super::computer::Computer;

// #[derive(Default)]
pub struct FluentComputerBuilder {
    cpu: Option<String>,
    memory: Option<u32>,
    storage: Option<u32>,
    gpu: Option<String>,
}

impl Default for FluentComputerBuilder {
    fn default() -> Self {
        Self {
            cpu: None,
            memory: Some(8),
            storage: Some(256),
            gpu: None,
        }
    }
}
impl FluentComputerBuilder {
    pub fn cpu(mut self, cpu: &str) -> Self {
        self.cpu = Some(cpu.to_string());
        self
    }

    pub fn memory(mut self, gb: u32) -> Self {
        self.memory = Some(gb);
        self
    }

    pub fn storage(mut self, gb: u32) -> Self {
        self.storage = Some(gb);
        self
    }

    pub fn gpu(mut self, gpu: &str) -> Self {
        self.gpu = Some(gpu.to_string());
        self
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
