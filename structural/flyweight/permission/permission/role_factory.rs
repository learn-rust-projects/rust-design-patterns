use super::role::Role;
use std::collections::HashMap;
use std::sync::Arc;
use std::sync::Mutex;

/// `RoleFactory` is responsible for managing and reusing `Role` instances.
/// It ensures that each role is created only once and then shared.
pub struct RoleFactory {
    roles: Arc<Mutex<HashMap<String, Arc<Role>>>>, // Shared roles using Arc
}
impl Default for RoleFactory {
    fn default() -> Self {
        Self::new()
    }
}
impl RoleFactory {
    /// Create a new RoleFactory.
    pub fn new() -> Self {
        Self {
            roles: Arc::new(Mutex::new(HashMap::new())),
        }
    }
    /// Retrieve a role with the specified name. If it doesn't exist, create it.
    pub fn get_role(&self, name: &str) -> Arc<Role> {
        let mut map = self.roles.lock().unwrap();
        map.entry(name.to_string())
            .or_insert_with(|| match name {
                "admin" => Arc::new(Role::new("admin", vec!["read", "write", "delete"])),
                "editor" => Arc::new(Role::new("editor", vec!["read", "write"])),
                "viewer" => Arc::new(Role::new("viewer", vec!["read"])),
                _ => Arc::new(Role::new("guest", vec![])),
            })
            .clone()
    }
}
