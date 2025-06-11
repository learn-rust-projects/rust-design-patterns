// main.rs
mod computer_builder;

use computer_builder::{FluentComputerBuilder, NonFluentComputerBuilder};

fn main() {
    // Fluent interface
    let computer = FluentComputerBuilder::default()
        .cpu("Intel i7")
        .memory(16)
        .storage(512)
        .gpu("NVIDIA RTX 3080")
        .build();

    println!("Using fluent builder:");
    computer.spec();

    // Non-fluent interface
    let mut non_fluent_builder = NonFluentComputerBuilder::default();
    non_fluent_builder.set_cpu("AMD Ryzen 9");
    non_fluent_builder.set_memory(32);
    non_fluent_builder.set_storage(1024);
    non_fluent_builder.set_gpu("None");
    let computer2 = non_fluent_builder.build();

    println!("Using non-fluent builder:");
    computer2.spec();
}
