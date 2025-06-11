use super::printer_trait::Printer;
pub struct HelloPrinter;

impl Printer for HelloPrinter {
    fn print(&self, _: &str) {
        println!("Hello, world!");
    }
}
