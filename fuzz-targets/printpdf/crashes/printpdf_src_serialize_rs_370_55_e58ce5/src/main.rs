use printpdf::PdfDocument;
use printpdf::serialize::PdfSaveOptions;

fn main() {
    let mut pdf_document = PdfDocument::default();
    let _ = pdf_document.add_bookmark("", 0usize);
    pdf_document.save_writer::<Vec<u8>>(&mut vec![], &PdfSaveOptions { optimize: false, subset_fonts: false, secure: false, image_optimization: None }, &mut vec![]);
}
