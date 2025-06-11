mod printer;

use printer::random_factory;

fn main() {
    let factory = random_factory();
    factory.print("Rust is awesome!");
}
