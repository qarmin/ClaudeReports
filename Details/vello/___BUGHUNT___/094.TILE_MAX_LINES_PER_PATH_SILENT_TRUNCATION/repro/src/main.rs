//! Real end-to-end reproduction for candidate 094.
//!
//! Uses `vello_cpu`'s public, ordinary rendering API (`RenderContext::fill_path` +
//! `RenderContext::render`) exactly the way any real caller would - the same public entry
//! point every vello_cpu-backed application uses to fill a path. No private/internal
//! function (`Tiles::make_tiles_analytic_aa`, `StripGenerator`, ...) is called directly.
//!
//! The path is an ordinary single `QuadTo` curve (the same shape a font glyph outline commonly
//! uses), scaled by an extreme affine transform. This is the same "zoom" mechanism this
//! workspace's own candidate 012 (QUADTO_ZOOM_SUBDIVISION_HANG) exploited, but at a scale chosen
//! to land the flattened line count just *above* `vello_common::tile::MAX_LINES_PER_PATH`
//! (134,217,728) rather than at the astronomical scales that cause an outright hang/OOM abort -
//! i.e. exactly the "large-but-not-crazy" input this candidate is about.
//!
//! In a debug build (debug-assertions on), this panics inside `tile.rs`'s
//! `debug_assert!(lines.len() <= MAX_LINES_PER_PATH, ...)`.
//! In a release build (debug-assertions off, vello_cpu's normal deployment configuration), the
//! same call completes with no error/panic of any kind and silently discards every line beyond
//! the limit before further processing - the pixmap comes back looking like a normal, successful
//! render, no different from any other program a caller would treat as "nothing went wrong".

use std::time::Instant;
use vello_cpu::color::palette::css;
use vello_cpu::kurbo::{Affine, BezPath, PathEl};
use vello_cpu::{Pixmap, RenderContext, Resources};

fn extreme_quad_path() -> BezPath {
    let mut path = BezPath::new();
    path.push(PathEl::MoveTo((0.0, 0.0).into()));
    path.push(PathEl::QuadTo((0.5, 1.0).into(), (1.0, 0.0).into()));
    path.push(PathEl::ClosePath);
    path
}

fn main() {
    // Only relevant to observing the *patched* behavior (a `log::warn!` line) - irrelevant to
    // the bug itself, which manifests identically whether or not a logger is installed.
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("warn")).init();

    // Independently confirm, via vello_common's own public flattening API, exactly how many
    // line segments this path+transform combination produces - so the reader can see the
    // over-the-limit line count without needing to instrument the crate.
    {
        use vello_common::fearless_simd::Level;
        use vello_common::flatten::{self, FlattenCtx, Line};
        use vello_common::geometry::RectU16;
        use vello_common::tile::MAX_LINES_PER_PATH;

        let mut line_buf: Vec<Line> = Vec::new();
        let mut ctx = FlattenCtx::default();
        flatten::fill(
            Level::baseline(),
            extreme_quad_path(),
            Affine::scale(1.2e16),
            &mut line_buf,
            &mut ctx,
            RectU16::new(0, 0, 100, 100),
        );
        println!(
            "flatten::fill produced {} lines (MAX_LINES_PER_PATH = {}, over by {})",
            line_buf.len(),
            MAX_LINES_PER_PATH,
            line_buf.len() as i64 - i64::from(MAX_LINES_PER_PATH)
        );
    }

    // Now exercise the real, public, ordinary vello_cpu rendering API end to end - exactly what
    // an application filling this path would call.
    let width = 100_u16;
    let height = 100_u16;
    let mut ctx = RenderContext::new(width, height);
    let mut resources = Resources::new();

    ctx.set_transform(Affine::scale(1.2e16));
    ctx.set_paint(css::MAGENTA);

    let start = Instant::now();
    ctx.fill_path(&extreme_quad_path());
    ctx.flush();

    let mut target = Pixmap::new(width, height);
    ctx.render(&mut target, &mut resources);
    let elapsed = start.elapsed();

    println!(
        "RenderContext::fill_path + render completed with NO panic/error in {elapsed:?} \
         (this line only prints if debug_assert! did not fire, i.e. a release-profile build)"
    );
}
