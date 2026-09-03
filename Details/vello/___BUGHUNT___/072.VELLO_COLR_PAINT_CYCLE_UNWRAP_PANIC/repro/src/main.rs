// Reproduction for candidate 072: VELLO_COLR_PAINT_CYCLE_UNWRAP_PANIC.
//
// `vello::scene`'s `EmojiLikeGlyph::Colr` handling (the code path `Scene::draw_glyphs(&font)
// .draw(..)` takes for a COLRv1 glyph) calls `colr.paint(location, &mut DrawColorGlyphs { .. })
// .unwrap()`. `colr.paint` is `skrifa::color::ColorGlyph::paint`, which returns
// `Result<(), skrifa::color::PaintError>` - and `PaintError` has variants
// (`PaintCycleDetected`, `DepthLimitExceeded`) that `skrifa` returns as a *safe failure signal*
// when a COLRv1 glyph's paint graph is self-referential (a `PaintColrGlyph` cycle) or too deep -
// this is spec-mandated cycle/depth protection in `skrifa`'s own traversal, not a sign of a
// corrupt/unparseable font. `vello`'s own sibling implementation for the same COLRv1 format
// (`glifo`, used by `vello_cpu`/`vello_hybrid`) already handles this by ignoring the error
// (`let _ = skrifa_glyph.paint(location_ref, self);`) and draining any layers left open - but
// classic `vello`'s own, separate `DrawColorGlyphs`/`Scene::draw_glyphs` implementation unwraps
// it unconditionally, so any real font containing a legitimately-encoded COLRv1 paint cycle
// panics the entire render call.
//
// This drives the REAL end-to-end path: classic vello's real public
// `Scene::draw_glyphs(&font).draw(..)` entry point, fed the real, unmodified
// `test_glyphs-glyf_colr_1.ttf` (Google/nanoemoji's COLRv1 test font, already shipped in this
// repo and already used by this project's own `vello_sparse_tests` COLR snapshot tests), on its
// own `paintcolrglyph_cycle_first`/`paintcolrglyph_cycle_second` glyphs - two glyphs the font's
// own authors deliberately encoded with a self-referential `PaintColrGlyph` cycle specifically to
// exercise this exact edge case. No wgpu/GPU device is needed at all: the panic happens while
// building the `Scene` (CPU-side), before any GPU rendering would occur.
//
// This is a single-participant case per CONFIRMATION.md section 11: a font file is a single
// external artifact handed to the renderer's real public entry point; no second participant is
// faked.

use std::iter;
use std::panic;
use std::sync::Arc;

const FONT_BYTES: &[u8] = include_bytes!("../assets/test_glyphs-glyf_colr_1.ttf");
/// `paintcolrglyph_cycle_first` / `paintcolrglyph_cycle_second` in
/// `test_glyphs-glyf_colr_1.ttf` - a real, deliberately-crafted mutually-referential pair of
/// `PaintColrGlyph` nodes (found via `fontTools` introspection of the font's `COLR` table, see
/// RESULT.md). `no_cycle_multi_colrglyph` is a sanity-check negative control: it also uses
/// `PaintColrGlyph` (nested, multiple times) but with no cycle, so it must NOT panic either
/// before or after the fix.
const CYCLE_GLYPH_IDS: [(u32, &str); 2] = [
    (178, "paintcolrglyph_cycle_first"),
    (179, "paintcolrglyph_cycle_second"),
];
const NO_CYCLE_CONTROL_GLYPH: (u32, &str) = (180, "no_cycle_multi_colrglyph");

fn draw_glyph(glyph_id: u32) {
    use vello::kurbo::Affine;
    use vello::peniko::{Blob, Fill, FontData};
    use vello::{Glyph, Scene};

    let font = FontData::new(Blob::new(Arc::new(FONT_BYTES.to_vec())), 0);

    let mut scene = Scene::new();
    scene
        .draw_glyphs(&font)
        .font_size(1000.0)
        .transform(Affine::translate((0.0, 1000.0)))
        .draw(
            Fill::NonZero,
            iter::once(Glyph {
                id: glyph_id,
                x: 0.0,
                y: 0.0,
            }),
        );
    // Touch the built encoding so the whole call isn't optimized away.
    std::hint::black_box(scene.encoding());
}

fn main() {
    let mut any_failed = false;

    for (glyph_id, name) in CYCLE_GLYPH_IDS {
        let mut successes = 0;
        let mut panics = 0;
        for _ in 0..3 {
            let result = panic::catch_unwind(|| draw_glyph(glyph_id));
            match result {
                Ok(()) => successes += 1,
                Err(payload) => {
                    panics += 1;
                    let msg = payload
                        .downcast_ref::<String>()
                        .cloned()
                        .or_else(|| payload.downcast_ref::<&str>().map(|s| (*s).to_owned()))
                        .unwrap_or_default();
                    println!("glyph {glyph_id} ({name}): PANIC - {msg}");
                }
            }
        }
        println!(
            "glyph {glyph_id} ({name}): {successes}/3 succeeded, {panics}/3 panicked"
        );
        if panics > 0 {
            any_failed = true;
        }
    }

    // Negative control: a font that uses PaintColrGlyph nesting WITHOUT a cycle must render fine.
    let (control_id, control_name) = NO_CYCLE_CONTROL_GLYPH;
    match panic::catch_unwind(|| draw_glyph(control_id)) {
        Ok(()) => println!("glyph {control_id} ({control_name}): OK (negative control, as expected)"),
        Err(_) => {
            println!(
                "glyph {control_id} ({control_name}): PANIC (unexpected - this glyph has no cycle)"
            );
            any_failed = true;
        }
    }

    if any_failed {
        eprintln!(
            "REPRODUCED - classic vello's real Scene::draw_glyphs(..).draw(..) panics on a real \
             COLRv1 font's deliberately-crafted PaintColrGlyph cycle glyph (colr.paint(..).unwrap() \
             on skrifa's PaintError::PaintCycleDetected)"
        );
        std::process::exit(1);
    }
    println!("did not reproduce - no panic for any glyph");
}
