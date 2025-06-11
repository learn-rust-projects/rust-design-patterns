use super::printer_trait::Printer;

pub struct RepeatPrinter;

impl Printer for RepeatPrinter {
    fn print(&self, s: &str) {
        println!("Repeat: {s}");
    }
}
