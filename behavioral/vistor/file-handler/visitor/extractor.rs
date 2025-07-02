use super::Visitor;
use crate::file::{PdfFile, PptFile, WordFile};

pub struct Extractor;

impl Visitor for Extractor {
    fn visit_pdf(&self, file: &PdfFile) {
        println!("Extracting PDF: {}", file.path);
    }
    fn visit_word(&self, file: &WordFile) {
        println!("Extracting Word: {}", file.path);
    }
    fn visit_ppt(&self, file: &PptFile) {
        println!("Extracting PPT: {}", file.path);
    }
}
