mod hello_printer;
mod hello_printer_factory;
mod printer_trait;
mod repeat_printer;
mod repeat_printer_factory;

pub use hello_printer_factory::HelloPrinterFactory;
pub use printer_trait::Printer;
pub use repeat_printer_factory::RepeatPrinterFactory;

// factory trait
pub trait PrinterFactory {
    fn create_printer(&self) -> Box<dyn Printer>;
    fn print(&self, s: &str) {
        self.create_printer().print(s)
    }
}

// Method to randomly create different factories
use rand::{Rng, rng};

pub fn random_factory() -> Box<dyn PrinterFactory> {
    let mut rng = rng();
    if rng.random_bool(0.5) {
        Box::new(HelloPrinterFactory)
    } else {
        Box::new(RepeatPrinterFactory)
    }
}
