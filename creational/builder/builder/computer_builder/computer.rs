// struct Computer
pub struct Computer {
    pub cpu: String,
    pub memory: u32,
    pub storage: u32,
    pub gpu: Option<String>,
}

impl Computer {
    pub fn spec(&self) {
        println!(
            "CPU: {}, Memory: {}GB, Storage: {}GB, GPU: {}",
            self.cpu,
            self.memory,
            self.storage,
            self.gpu.as_deref().unwrap_or("None")
        );
    }
}
