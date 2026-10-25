use hayro::PixmapSettings;
use hayro::RenderCache;
use hayro::RenderSettings;
use hayro_interpret::InterpreterSettings;
use hayro_jbig2::ParseError;
use hayro_syntax::Pdf;
use hayro_syntax::PdfData;

fn main() {
    let _ = ParseError::UnexpectedEof;
    let pdf = Pdf::new(PdfData::from(include_bytes!("document_0.pdf").to_vec())).unwrap();
    let pages = pdf.pages();
    let page = pages.get(0usize).unwrap();
    let render_cache = RenderCache::new();
    let _ = hayro::render(page, &render_cache, &InterpreterSettings::default(), &RenderSettings::default(), &PixmapSettings::default());
}
