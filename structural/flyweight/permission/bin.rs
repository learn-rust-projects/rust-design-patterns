use flyweight_permission::permission::RoleFactory;
use flyweight_permission::permission::User;

fn main() {
    let factory = RoleFactory::new();

    let admin_role = factory.get_role("admin");
    let viewer_role = factory.get_role("viewer");

    let user1 = User::new(1, "Alice", admin_role.clone());
    let user2 = User::new(2, "Bob", viewer_role.clone());
    let user3 = User::new(3, "Charlie", admin_role.clone()); // Reuse

    user1.print_profile();
    user2.print_profile();
    user3.print_profile();
}
