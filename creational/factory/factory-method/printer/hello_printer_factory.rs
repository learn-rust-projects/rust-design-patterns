use super::hello_printer::HelloPrinter;
use super::printer_trait::Printer;

pub struct HelloPrinterFactory;

impl super::PrinterFactory for HelloPrinterFactory {
    fn create_printer(&self) -> Box<dyn Printer> {
        Box::new(HelloPrinter)
    }
}
