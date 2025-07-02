pub mod compressor;
pub mod extractor;

use crate::file::{PdfFile, PptFile, WordFile};

pub trait Visitor {
    fn visit_pdf(&self, file: &PdfFile);
    fn visit_word(&self, file: &WordFile);
    fn visit_ppt(&self, file: &PptFile);
}

pub use compressor::Compressor;
pub use extractor::Extractor;
