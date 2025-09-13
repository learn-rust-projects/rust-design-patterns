use vistor_file_handler::{Compressor, Extractor, File, PdfFile, PptFile, WordFile};

fn main() {
    let pdf = PdfFile {
        path: "a.pdf".into(),
    };
    let word = WordFile {
        path: "b.word".into(),
    };
    let ppt = PptFile {
        path: "c.ppt".into(),
    };

    let files = vec![File::Pdf(&pdf), File::Word(&word), File::Ppt(&ppt)];

    let extractor = Extractor;
    let compressor = Compressor;

    println!("Extracting files:");
    for file in &files {
        file.accept(&extractor);
    }

    println!("\nCompressing files:");
    for file in &files {
        file.accept(&compressor);
    }
}
