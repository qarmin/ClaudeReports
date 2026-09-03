// Reproduction for candidate 054: `feOffset` (FilterPrimitive::Offset) with a large negative
// `dx`/`dy` (magnitude greater than u16::MAX) leaves the source pixels stale (not cleared to
// transparent) instead of moving them off-canvas as it should.
//
// This drives the real, public `vello_cpu::RenderContext` API end-to-end: push a real
// `feOffset` filter layer via `push_filter_layer`, fill a small opaque rectangle inside it, pop
// the layer, and rasterize to a real pixmap - then checks whether the rectangle's original
// pixels are still opaque even though the huge offset should have moved the whole rectangle far
// outside the visible canvas (making the original location fully transparent).
//
// Root cause (sparse_strips/vello_cpu/src/filter/shift.rs, `process_offset_pixel`):
// `should_clear` computes `x >= (width as i32 + dx_pixels) as u16`. When `dx_pixels` is negative
// and its magnitude exceeds the filter layer's internal pixmap width, `width as i32 + dx_pixels`
// is itself negative, and casting a negative `i32` to `u16` with `as` wraps (e.g. -6i32 as u16 ==
// 65530) instead of saturating. Since a real `x: u16` can never be >= that huge wrapped value,
// `should_clear` becomes wrongly `false`, so the original (now off-canvas) pixel keeps its color
// instead of becoming transparent.
//
// This is reachable through the real filter pipeline because the internal filter-layer pixmap's
// padding is derived from `dx`/`dy` through `vello_common`'s `snapped_padding`, which casts the
// padding amount down to `u16` with `as` too - but that's a float-to-int cast, which *saturates*
// (caps at 65535) rather than wrapping. So once `|dx|` exceeds 65535, the padding added to the
// pixmap is capped at 65535 while `dx_pixels` (an `i32`, not capped until ~2^31) is not - and the
// pixmap ends up narrower than `|dx_pixels|`, triggering the wraparound bug above.

use vello_common::color::palette::css::REBECCA_PURPLE;
use vello_common::filter_effects::{Filter, FilterPrimitive};
use vello_common::kurbo::Rect;
use vello_cpu::{Pixmap, RasterizerSettings, RenderContext, Resources};

/// Renders a small opaque rectangle inside an `feOffset` filter layer with the given `dx`, and
/// returns the sampled pixels at the rectangle's *original* location after rendering.
fn render_offset(dx: f32) -> Vec<vello_common::peniko::color::PremulRgba8> {
    let width: u16 = 100;
    let height: u16 = 20;

    let mut ctx = RenderContext::new(width, height);
    let mut resources = Resources::new();

    let filter = Filter::from_primitive(FilterPrimitive::Offset { dx, dy: 0.0 });
    let rect = Rect::new(40.0, 5.0, 60.0, 15.0);

    ctx.push_filter_layer(filter);
    ctx.set_paint(REBECCA_PURPLE);
    ctx.fill_rect(&rect);
    ctx.pop_layer();

    let mut pixmap = Pixmap::new(width, height);
    ctx.render_with(&mut pixmap, &mut resources, RasterizerSettings::default());

    [40u16, 45, 50, 55, 59]
        .iter()
        .map(|&x| pixmap.sample(x, 10))
        .collect()
}

fn main() {
    // Sanity check: a "normal" large-but-in-range negative offset correctly clears the source
    // rectangle's original location (moves it off-canvas to the left).
    let sane = render_offset(-30.0);
    println!("dx = -30:    {sane:?}");
    let sane_all_transparent = sane.iter().all(|p| p.a == 0);
    if !sane_all_transparent {
        eprintln!("UNEXPECTED: dx = -30 did not clear the source rectangle - test setup is broken");
        std::process::exit(2);
    }

    // The actual trigger: a negative offset whose magnitude exceeds u16::MAX, so the filter
    // layer's padding computation (capped at u16::MAX via a saturating float->int cast) can no
    // longer keep the internal pixmap wide enough for the raw (uncapped) `dx_pixels` value.
    let buggy = render_offset(-70_000.0);
    println!("dx = -70000: {buggy:?}");
    let buggy_all_transparent = buggy.iter().all(|p| p.a == 0);

    if !buggy_all_transparent {
        eprintln!(
            "REPRODUCED: source rectangle at its original location is still opaque after being \
             offset by dx = -70000 (should be fully transparent, exactly like the dx = -30 case)."
        );
        std::process::exit(1);
    }

    println!("did not reproduce - stale pixels were correctly cleared");
}
