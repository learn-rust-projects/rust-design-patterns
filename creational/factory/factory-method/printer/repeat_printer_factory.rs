use super::{printer_trait::Printer, repeat_printer::RepeatPrinter};

pub struct RepeatPrinterFactory;

impl super::PrinterFactory for RepeatPrinterFactory {
    fn create_printer(&self) -> Box<dyn Printer> {
        Box::new(RepeatPrinter)
    }
}
