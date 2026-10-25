use hayro::PixmapSettings;
use hayro::RenderCache;
use hayro::RenderSettings;
use hayro_interpret::InterpreterSettings;
use hayro_syntax::Pdf;
use hayro_syntax::PdfData;

fn main() {
    let pdf = Pdf::new_with_password(PdfData::from(include_bytes!("data_0.bin").to_vec()), "").unwrap();
    let pages = pdf.pages();
    let page = pages.get(1usize).unwrap();
    let render_cache = RenderCache::new();
    let _ = hayro::render(page, &render_cache, &InterpreterSettings::default(), &RenderSettings { force_image_interpolation: false }, &PixmapSettings::default());
}
