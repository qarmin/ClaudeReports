//! Reproduces a panic reachable through `vello_cpu`'s real public rendering API when a filter's
//! (blur/drop-shadow) std_deviation, combined with the current transform's scale, produces a
//! device-space expansion whose padding, after `FilterData::new`'s saturating f64->u16 cast
//! (`sparse_strips/vello_common/src/filter/mod.rs`, `snapped_padding`), reaches exactly
//! `u16::MAX`. `ViewportState::push_root_viewport` (`sparse_strips/vello_common/src/viewport.rs`)
//! then constructs an internal filter-layer `StripGenerator` whose width/height is exactly
//! `u16::MAX` - a value that is not a multiple of `Tile::WIDTH`/`Tile::HEIGHT` (4), so its last
//! tile's right/bottom edge cannot be represented as a `u16` pixel coordinate. Drawing ordinary
//! content inside that internal filter surface (via the real `RenderContext::fill_path`) then
//! panics inside `vello_common::util::strip_bbox`'s `checked_mul(Tile::WIDTH).unwrap()`.
//!
//! This drives the real, public, production entry point exactly as a real application would
//! (`RenderContext::push_layer` with `filter: Some(...)`, `fill_path`, `pop_layer`,
//! `render_with` - the same sequence `sparse_strips/vello_cpu/examples/basic.rs` uses for the
//! rest of the public API). The caller-supplied `std_deviation` and `transform` are a single
//! external artifact with no second participant to fake (CONFIRMATION.md section 11's
//! single-process exception applies directly here, the same as candidates 019 and 035 in this
//! workspace).
//!
//! Case 1 uses an identity transform with an extreme `std_deviation`. Case 2 (a differently
//! shaped input in the same bug class, not just the same byte-for-byte trigger) uses a moderate
//! `std_deviation` combined with a large transform scale instead - `transform_blur_params`
//! multiplies the two together (`std_deviation * uniform_scale`), so either one alone, if large
//! enough, reaches the same `u16::MAX` boundary.
//!
//! Exits normally (prints "REPRO: no panic in any case (bug fixed or not reproduced)") if
//! patched; panics (non-zero exit, backtrace-visible panic location) if run against the
//! unpatched tree.

use vello_common::filter_effects::{Filter, FilterFunction};
use vello_cpu::color::palette::css::RED;
use vello_cpu::kurbo::{Affine, Rect, Shape};
use vello_cpu::{Pixmap, RenderContext, Resources};

fn run_case(width: u16, height: u16, transform_scale: f64, std_deviation: f32, label: &str) {
    println!("--- case: {label} ---");
    let mut ctx = RenderContext::new(width, height);
    let mut resources = Resources::new();

    ctx.set_transform(Affine::scale(transform_scale));
    ctx.push_layer(
        None,
        None,
        None,
        None,
        Some(Filter::from_function(FilterFunction::Blur {
            radius: std_deviation,
        })),
    );
    ctx.set_paint(RED);
    // An ordinary, small shape near the origin - nowhere near any edge of the *user-requested*
    // canvas. The eventual out-of-range coordinate is entirely internal, produced by the
    // saturated filter padding, not by anything the caller did unusually here.
    ctx.fill_path(&Rect::new(0.0, 0.0, 10.0, 10.0).to_path(0.1));
    ctx.pop_layer();
    ctx.flush();

    let mut pixmap = Pixmap::new(width, height);
    ctx.render_with(&mut pixmap, &mut resources, Default::default());
    println!("  rendered OK, no panic ({width}x{height} canvas).");
}

fn main() {
    // Case 1: extreme std_deviation, identity-scale transform.
    // Device-space expansion = 3 * 30_000 * 1.0 = 90_000 > u16::MAX (65_535).
    run_case(200, 200, 1.0, 30_000.0, "extreme std_deviation, identity transform");

    // Case 2: moderate std_deviation, large transform scale (a differently-shaped input in the
    // same class - CONFIRMATION.md section 7).
    // Device-space expansion = 3 * 50 * 500 = 75_000 > u16::MAX.
    run_case(300, 300, 500.0, 50.0, "moderate std_deviation, large transform scale");

    println!("\nREPRO: no panic in any case (bug fixed or not reproduced).");
}
