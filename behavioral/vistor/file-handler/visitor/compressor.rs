use super::Visitor;
use crate::file::{PdfFile, PptFile, WordFile};

pub struct Compressor;

impl Visitor for Compressor {
    fn visit_pdf(&self, file: &PdfFile) {
        println!("Compressing PDF: {}", file.path);
    }
    fn visit_word(&self, file: &WordFile) {
        println!("Compressing Word: {}", file.path);
    }
    fn visit_ppt(&self, file: &PptFile) {
        println!("Compressing PPT: {}", file.path);
    }
}
