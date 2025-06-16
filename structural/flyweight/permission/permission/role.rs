#[derive(Debug)]
pub struct Role {
    name: String,
    permissions: Vec<String>, // e.g. ["read", "write", "delete"]
}

impl Role {
    pub fn new(name: &str, permissions: Vec<&str>) -> Self {
        Role {
            name: name.to_string(),
            permissions: permissions.iter().map(|s| s.to_string()).collect(),
        }
    }

    pub fn describe(&self) {
        println!("Role: {}, Permissions: {:?}", self.name, self.permissions);
    }
}
