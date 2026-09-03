//! Real end-to-end reproduction for candidate 083
//! (DEPTH_BUFFER_BUCKET_SPAN_U16_MULTIPLICATION_OVERFLOW).
//!
//! Drives the actual public `vello_cpu::RenderContext` rendering pipeline (the same
//! `set_paint`/`fill_rect`/`flush`/`render` sequence a real application uses) at a render
//! target width close to `u16::MAX`.
//!
//! `vello_cpu::coarse::depth::DepthBuffer::new(buffer_width)` sizes its internal bucket
//! array as `buffer_width.div_ceil(DEPTH_BUCKET_WIDTH)` (`DEPTH_BUCKET_WIDTH == 128`). For a
//! (tile-aligned) `buffer_width` in `[65409, 65532]` (the top of the achievable range given
//! `u16::MAX == 65535` and tile-width-4 snapping), this yields a `bucket_count` of exactly
//! `512`. Every strip row's rasterization unconditionally calls
//! `Fine::init_uncovered_range`, which asks the (per-row, fully cleared) depth buffer for
//! every "unset" (still-zero) bucket run via `DepthBuffer::for_each_unset_run`. For a row
//! with *no* opaque depth-tracked draws at all (the common case - even a blank row, or a
//! row that only has non-opaque/masked/layered fills), the *entire* buffer (all 512
//! buckets) comes back as a single unset run, i.e. `bucket_span(0, 512)`, which computes
//! `(512 - 0) as u16 * DEPTH_BUCKET_WIDTH` == `512u16 * 128u16` == `65536` - one past
//! `u16::MAX`.
//!
//! - In a debug build (`overflow-checks = true`, the crate's own default `dev` profile),
//!   this panics with "attempt to multiply with overflow".
//! - In a release build (`overflow-checks = false`), it silently wraps to `0`, producing a
//!   degenerate zero-width `Span`. `Span::intersect` then returns `None` for that span
//!   (`0 < 0` is false), so `for_each_unset_run`'s callback is never invoked for that
//!   row at all - meaning the row's "clear to background" step is skipped entirely. Since
//!   `Fine`'s base blend buffer is allocated once per `render()` call and reused across
//!   every strip row (not reallocated per row), a row that follows an opaque-filled row
//!   silently inherits ("bleeds") that row's leftover pixel data instead of being cleared.
//!
//! This repro demonstrates both manifestations against the same scene (a two-row canvas:
//! row 0 fully opaque red, row 1 left blank) by running the real `render()` call inside
//! `catch_unwind` and checking the actual output pixels either way.

use std::panic;

use vello_cpu::color::palette::css::RED;
use vello_cpu::kurbo::Rect;
use vello_cpu::{Pixmap, RenderContext, Resources};

/// Largest multiple of `Tile::WIDTH` (4) that is `<= u16::MAX`, and tile-aligned so
/// `RenderContext`'s internal viewport-snapping doesn't shrink it further. Together with
/// `DEPTH_BUCKET_WIDTH == 128`, this yields a `bucket_count` of exactly 512
/// (`65532.div_ceil(128) == 512`), the smallest width already large enough to trigger a
/// `512u16 * 128u16` overflow.
const WIDTH: u16 = 65532;
const HEIGHT: u16 = 8; // two Tile::HEIGHT(4)-tall strip rows.

fn render_once() -> Pixmap {
    let mut ctx = RenderContext::new(WIDTH, HEIGHT);

    // Row 0 (y in 0..4): full-width opaque fill - the recorded draw that becomes a
    // depth-tracked "opaque" command, driving `DepthBuffer`/`Fine::init_uncovered_range`
    // for every row of this render, including row 1 below.
    ctx.set_paint(RED);
    ctx.fill_rect(&Rect::new(0.0, 0.0, f64::from(WIDTH), 4.0));
    ctx.flush();
    // Row 1 (y in 4..8): left intentionally blank - must render fully transparent.

    let mut resources = Resources::new();
    let mut pixmap = Pixmap::new(WIDTH, HEIGHT);
    ctx.render(&mut pixmap, &mut resources);
    pixmap
}

fn main() {
    let result = panic::catch_unwind(render_once);

    match result {
        Err(payload) => {
            let msg = payload
                .downcast_ref::<String>()
                .map(String::as_str)
                .or_else(|| payload.downcast_ref::<&str>().copied())
                .unwrap_or("<non-string panic payload>");
            println!(
                "PANIC: RenderContext::render() panicked (bug reproduced, debug-assertions build): {msg}"
            );
            std::process::exit(42);
        }
        Ok(pixmap) => {
            let red_pixel = RED.premultiply().to_rgba8();
            let bled_count = (0..WIDTH)
                .filter(|&x| pixmap.sample(x, 4) == red_pixel)
                .count();

            if bled_count > 0 {
                println!(
                    "WRONG OUTPUT: row 1 (should be fully transparent) has {bled_count}/{WIDTH} pixels showing row 0's red content bled through (bug reproduced, release-mode logic corruption)"
                );
                std::process::exit(42);
            }

            println!(
                "OK: RenderContext::render() returned normally and row 1 is fully transparent (bug not present / fixed)"
            );
        }
    }
}
