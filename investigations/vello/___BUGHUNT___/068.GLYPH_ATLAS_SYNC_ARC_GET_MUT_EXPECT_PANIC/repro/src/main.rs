// Standalone reproduction for candidate 068
// (GLYPH_ATLAS_SYNC_ARC_GET_MUT_EXPECT_PANIC).
//
// Drives `vello_cpu`'s real, public API end-to-end (`RenderContext`, `Resources`,
// `register_image`/`destroy_image`, `glyph_run`/`atlas_cache`, `render_with`) - no internal
// (`pub(crate)`) types are touched.
//
// The scenario is entirely ordinary, documented usage: a single, persistent `Resources`
// instance (the crate's own doc comment says "You should create one such instance per
// renderer") reused across multiple frames/`RenderContext`s, exactly as a real embedder would.
//
// Frame 1 draws one glyph through the atlas cache *and* a rectangle painted with an
// `ImageSource::OpaqueId` that points at an image the caller already destroyed - a realistic,
// easy-to-hit caller mistake (forgetting that a still-referenced image was already evicted).
// This makes `render_with` panic partway through rasterization - after it already registered
// the glyph atlas's page(s) into the (private) image registry, but before it gets a chance to
// remove them again. The panic is caught with `catch_unwind`, simulating a caller that guards
// individual frames against panics so one bad frame doesn't take down the whole process/app (a
// common, legitimate defensive pattern for a rendering library embedded in a larger app).
//
// Frame 2 is a brand new `RenderContext` - reusing the same (persistent) `Resources` - drawing
// a different, perfectly ordinary glyph. Nothing about frame 2 on its own is wrong. Yet on the
// pristine (unpatched) code, frame 2 panics anyway, because frame 1's panic left the image
// registry holding a stale clone of the atlas page's `Arc<Pixmap>`, permanently violating the
// "uniquely owned" invariant `sync_glyph_cache`'s `Arc::get_mut(...).expect(...)` calls assume.
//
// Run with `cargo run` (works in both debug and release - this is a plain, always-checked
// `Option::expect`, not something gated behind `debug_assertions`).

use std::panic::{AssertUnwindSafe, catch_unwind};
use std::sync::Arc;

use vello_cpu::kurbo::Rect;
use vello_cpu::peniko::{Blob, FontData, ImageQuality, ImageSampler};
use vello_cpu::{
    Glyph, Image, ImageSource, Pixmap, RasterizerSettings, RenderContext, Resources,
};

const ROBOTO_FONT: &[u8] = include_bytes!("../../clone/examples/assets/roboto/Roboto-Regular.ttf");
const NOTO_CBTF_FONT: &[u8] =
    include_bytes!("../../clone/examples/assets/noto_color_emoji/NotoColorEmoji-CBTF-Subset.ttf");

/// Runs the two-frame scenario described above for one font/glyph-id combination.
///
/// Returns `Ok(())` if frame 2 rendered successfully (bug fixed / not present), or `Err(msg)`
/// with the panic message if frame 2 itself panicked (bug present).
fn run_scenario(name: &str, font_bytes: &'static [u8], font_size: f32) -> Result<(), String> {
    println!("=== scenario: {name} ===");

    let font = FontData::new(Blob::new(Arc::new(font_bytes)), 0);
    let mut resources = Resources::new();

    // An image the caller registers and then destroys - realistic, ordinary lifecycle -
    // before it renders a frame that (by mistake) still references it.
    let bad_id = resources.register_image(Arc::new(Pixmap::new(4, 4)));
    resources.destroy_image(bad_id);

    let mut ctx1 = RenderContext::new(64, 64);
    ctx1.glyph_run(&mut resources, &font)
        .atlas_cache(true)
        .font_size(font_size)
        .fill_glyphs(
            [Glyph {
                id: 1,
                x: 0.0,
                y: 0.0,
            }]
            .into_iter(),
        );
    ctx1.set_paint(Image {
        image: ImageSource::opaque_id(bad_id),
        sampler: ImageSampler {
            quality: ImageQuality::Low,
            ..Default::default()
        },
    });
    ctx1.fill_rect(&Rect::new(0.0, 0.0, 4.0, 4.0));

    let mut pixmap1 = Pixmap::new(64, 64);
    let frame1 = catch_unwind(AssertUnwindSafe(|| {
        ctx1.render_with(&mut pixmap1, &mut resources, RasterizerSettings::default());
    }));
    match &frame1 {
        Ok(()) => {
            println!("  frame 1: did NOT panic (unexpected - the destroyed-image reference should have panicked)");
        }
        Err(payload) => {
            let msg = payload
                .downcast_ref::<String>()
                .cloned()
                .or_else(|| payload.downcast_ref::<&str>().map(|s| s.to_string()))
                .unwrap_or_else(|| "<non-string panic payload>".to_string());
            println!("  frame 1: panicked as expected (caught): {msg}");
        }
    }

    // Frame 2: fresh `RenderContext`, same persistent `Resources`, a different (but equally
    // ordinary) glyph - this alone is a ordinary, successful render.
    let mut ctx2 = RenderContext::new(64, 64);
    ctx2.glyph_run(&mut resources, &font)
        .atlas_cache(true)
        .font_size(font_size)
        .fill_glyphs(
            [Glyph {
                id: 2,
                x: 0.0,
                y: 0.0,
            }]
            .into_iter(),
        );

    let mut pixmap2 = Pixmap::new(64, 64);
    let frame2 = catch_unwind(AssertUnwindSafe(|| {
        ctx2.render_with(&mut pixmap2, &mut resources, RasterizerSettings::default());
    }));

    match frame2 {
        Ok(()) => {
            println!("  frame 2: rendered successfully - bug not present / fixed");
            Ok(())
        }
        Err(payload) => {
            let msg = payload
                .downcast_ref::<String>()
                .cloned()
                .or_else(|| payload.downcast_ref::<&str>().map(|s| s.to_string()))
                .unwrap_or_else(|| "<non-string panic payload>".to_string());
            println!("  frame 2: PANICKED (bug present): {msg}");
            Err(msg)
        }
    }
}

fn main() {
    // Two inputs in the same bug class (CONFIRMATION.md section 7): an outline/COLR glyph
    // (hits `sync_glyph_cache`'s "replay" `Arc::get_mut` call site), and a bitmap glyph (hits
    // the sibling "bitmap upload" `Arc::get_mut` call site).
    let outline_result = run_scenario("outline/COLR glyph (Roboto)", ROBOTO_FONT, 20.0);
    let bitmap_result = run_scenario("bitmap glyph (Noto Color Emoji CBTF)", NOTO_CBTF_FONT, 64.0);

    if outline_result.is_err() || bitmap_result.is_err() {
        println!("\nREPRODUCED: a later, unrelated frame's atlas sync panicked because an");
        println!("earlier frame's mid-render panic left the image registry holding a stale");
        println!("`Arc<Pixmap>` clone for a glyph atlas page.");
        std::process::exit(1);
    }

    println!("\ndid not reproduce: both scenarios' frame 2 rendered successfully");
}
