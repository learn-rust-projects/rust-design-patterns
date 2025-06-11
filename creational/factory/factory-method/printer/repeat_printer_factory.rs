use super::printer_trait::Printer;
use super::repeat_printer::RepeatPrinter;

pub struct RepeatPrinterFactory;

impl super::PrinterFactory for RepeatPrinterFactory {
    fn create_printer(&self) -> Box<dyn Printer> {
        Box::new(RepeatPrinter)
    }
}
