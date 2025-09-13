pub mod pdf;
pub mod ppt;
pub mod word;

pub use pdf::PdfFile;
pub use ppt::PptFile;
pub use word::WordFile;

use crate::visitor::Visitor;

pub enum File<'a> {
    Pdf(&'a PdfFile),
    Word(&'a WordFile),
    Ppt(&'a PptFile),
}

impl<'a> File<'a> {
    pub fn accept(&self, visitor: &dyn Visitor) {
        match self {
            File::Pdf(f) => visitor.visit_pdf(f),
            File::Word(f) => visitor.visit_word(f),
            File::Ppt(f) => visitor.visit_ppt(f),
        }
    }
}
