use hayro_syntax::Pdf;
use hayro_syntax::PdfData;

fn main() {
    let _ = Pdf::new_with_password(PdfData::from(include_bytes!("data_0.bin").to_vec()), "");
}
