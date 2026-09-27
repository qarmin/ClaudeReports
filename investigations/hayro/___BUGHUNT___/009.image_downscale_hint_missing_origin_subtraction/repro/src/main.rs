// Standalone reproduction for candidate 009: `hayro/src/image.rs::draw_pdf_image`'s
// CTM-derived downscale-resolution hint (`target_width`/`target_height`, passed to
// `RasterImage::with_rgba`/`StencilImage::with_stencil` so decoders like `hayro-jpeg2000` can
// skip decoding resolution levels the page will never show) computes
// `(transform * Point::new(image.width(), 0.0)).to_vec2().length()` - it treats the *absolute*
// transformed device-space point as if it were a vector from the coordinate origin, without
// subtracting the transformed image origin (`transform * Point::new(0.0, 0.0)`). Since `transform`
// always carries whatever translation the placement's `cm` matrix (and the page's own initial
// y-flip transform) contributes, the computed "target size" is systematically inflated by however
// far the image happens to be placed from device-space (0, 0) - for an ordinary placement (not at
// the literal top-left corner of the page) this can inflate the hint by an order of magnitude or
// more, defeating the resolution-skip optimization it exists for.
//
// This drives the real, public, top-level `hayro::render()` entry point end-to-end - the same
// function this project's own shipped `hayro/examples/render.rs` calls - on a real, legitimately-
// parseable PDF (classic xref table, real `/JPXDecode` image XObject wrapping a real JPEG2000
// codestream with genuine multi-resolution-level wavelet data, not a header-patched fake). Nothing
// about the trigger path is faked: an ordinary PDF places an ordinary small thumbnail of a larger
// real image somewhere on a normal-sized page, exactly the kind of layout any real-world PDF does
// routinely.
//
// The bug does not crash or panic - it is a silent performance/logic bug (the resolution-skip
// hint stays wrong, `hayro-jpeg2000` decodes far more resolution levels than the placement needs).
// The externally-observable symptom (without touching hayro's internals) is wall-clock time: with
// the hint computed correctly, decoding+rendering this thumbnail should take low-single-digit
// milliseconds; with the bug, `hayro-jpeg2000` decodes several extra wavelet resolution levels it
// didn't need to, costing several times longer for the exact same visible output.
//
// Exit code convention: this repro does not assert pass/fail on its own (see RESULT.md for why a
// hardcoded, machine-independent timing threshold isn't a reliable automated assertion for a raw
// performance-inflation bug) - it prints the measured median render time so `just test`'s
// before/after comparison can be read directly. RESULT.md/output.txt record the actual measured
// numbers from the session that confirmed this.

use hayro::hayro_syntax::Pdf;
use hayro::hayro_interpret::InterpreterSettings;
use hayro::{RenderCache, RenderSettings, render};
use std::time::{Duration, Instant};

// A real JPEG2000 codestream (JP2 box format) with genuine multi-resolution-level wavelet data:
// 1241x1752, grayscale, taken from this project's own IDWT benchmark test data
// (`___OPTIMIZATIONS___/002.hayro_jpeg2000_idwt_horizontal/bench/idwt/testdata/sample.jp2`).
// Unlike candidate 008's PoC, this is not hex-patched at all - every byte is a legitimate,
// fully-decodable JPEG2000 image, so the resolution-skip optimization has real work to actually
// skip.
const JP2: &[u8] = include_bytes!("sample.jp2");

/// Builds a minimal, real, classic-xref-table PDF that places a 60x85 thumbnail of the
/// (1241x1752) `JP2` image on a normal US-Letter page (612x792), at the given device-space
/// translation - `(500, 100)` is an entirely ordinary "photo somewhere on a page" placement, not
/// an edge case.
fn build_pdf(tx: i32, ty: i32) -> Vec<u8> {
    let img_dict = format!(
        "<< /Type /XObject /Subtype /Image /Width 1241 /Height 1752 \
/BitsPerComponent 8 /ColorSpace /DeviceGray /Filter /JPXDecode /Length {} >>",
        JP2.len()
    )
    .into_bytes();
    let mut obj4 = Vec::new();
    obj4.extend_from_slice(&img_dict);
    obj4.extend_from_slice(b"\nstream\n");
    obj4.extend_from_slice(JP2);
    obj4.extend_from_slice(b"\nendstream");

    let content = format!("q 60 0 0 85 {tx} {ty} cm /Im0 Do Q").into_bytes();
    let mut obj5 = format!("<< /Length {} >>\nstream\n", content.len()).into_bytes();
    obj5.extend_from_slice(&content);
    obj5.extend_from_slice(b"\nendstream");

    let objs: Vec<Vec<u8>> = vec![
        b"<< /Type /Catalog /Pages 2 0 R >>".to_vec(),
        b"<< /Type /Pages /Kids [3 0 R] /Count 1 >>".to_vec(),
        b"<< /Type /Page /Parent 2 0 R /MediaBox [0 0 612 792] /Resources << /XObject << /Im0 4 0 R >> >> /Contents 5 0 R >>".to_vec(),
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

fn median(mut v: Vec<Duration>) -> Duration {
    v.sort();
    v[v.len() / 2]
}

fn bench(label: &str, pdf_bytes: Vec<u8>, runs: usize) -> Duration {
    let pdf = Pdf::new(pdf_bytes).expect("PoC PDF should parse");
    let page = &pdf.pages()[0];
    let interpreter_settings = InterpreterSettings::default();
    let render_settings = RenderSettings::default();

    let mut durations = Vec::with_capacity(runs);
    for _ in 0..runs {
        // A fresh cache every run, so no run benefits from a previous run's decode cache.
        let cache = RenderCache::new();
        let start = Instant::now();
        let _pixmap = render(page, &cache, &interpreter_settings, &render_settings);
        durations.push(start.elapsed());
    }

    let med = median(durations.clone());
    println!(
        "{label}: runs={runs} median={med:?} all={:?}",
        durations.iter().map(|d| format!("{d:?}")).collect::<Vec<_>>()
    );
    med
}

fn main() {
    println!(
        "Real end-to-end trigger: hayro::render() (the same public entry point hayro/examples/render.rs \
uses) on a real PDF placing a 60x85 thumbnail of a real 1241x1752 multi-resolution-level JPEG2000 \
image on a normal 612x792 page."
    );

    // An ordinary, unremarkable placement: a thumbnail somewhere on the page, not pinned to the
    // literal top-left corner. This is the common case, not a contrived one.
    let realistic = build_pdf(500, 100);
    let realistic_time = bench("realistic thumbnail placement (tx=500, ty=100)", realistic, 5);

    // The same thumbnail, placed with a zero `cm` translation - included only as a second data
    // point showing the hint's value (and hence decode cost) shifts with placement at all, which
    // it should not for a correctly-computed *device-space size* hint.
    let near_origin = build_pdf(0, 0);
    let near_origin_time = bench("near-origin placement (tx=0, ty=0)", near_origin, 5);

    println!(
        "\nmedian(realistic)/median(near-origin) = {:.2}x",
        realistic_time.as_secs_f64() / near_origin_time.as_secs_f64().max(1e-9)
    );
    println!(
        "See this candidate's RESULT.md for the reference numbers captured on the session's own \
machine (unpatched vs patched) - a correct hint computation should make both placements decode \
to the same ~60x85 target resolution and take comparable, low-single-digit-millisecond time \
regardless of where on the page the thumbnail sits."
    );
}
