use std::sync::Arc;

use super::role::Role;
pub struct User {
    id: u32,
    username: String,
    role: Arc<Role>, // Flyweight 引用
}

impl User {
    pub fn new(id: u32, username: &str, role: Arc<Role>) -> Self {
        User {
            id,
            username: username.to_string(),
            role,
        }
    }

    pub fn print_profile(&self) {
        println!("User: {}, ID: {}", self.username, self.id);
        self.role.describe();
    }
}
