//! Standalone reproduction for vello issue #1752: "Zooming far in on nearly any demo scene hangs
//! the app".
//!
//! This calls the real, public `vello_common::flatten::fill` entry point - the same function
//! every vello renderer backend (`vello_cpu`, `vello_hybrid`) calls, via
//! `StripGenerator::generate_filled_path`, every time it fills a path (including glyph outlines,
//! which on most fonts are literally made of `QuadTo` curves - this is why "nearly any demo
//! scene" is affected as soon as it draws any text, not just specially crafted paths).
//!
//! The path here is an entirely ordinary-looking quadratic Bézier curve (the kind any font
//! outline or piece of vector art might contain) with completely unremarkable control points.
//! The only unusual thing is the transform: applying an affine transform with a large uniform
//! scale factor to it - exactly what "zooming far in" on a scene does to every path in it.
//!
//! `fill()`'s handling of `PathEl::QuadTo` derives a line-segment count `n` from a
//! curvature/subdivision estimate that grows with the transform's scale factor with no upper
//! bound, then runs a plain `for i in 1..n` loop emitting one line segment per iteration - no
//! allocation happens up front that could fail and abort the process (contrast with the cubic
//! (`CurveTo`) path in `flatten_cubic_simd`, which resizes a `Vec` up front, and can therefore
//! abort with an allocation failure for sufficiently extreme input - see this project's
//! `___BUGHUNT___/004.LARGE_CUBIC_UNBOUNDED_FLATTEN_ALLOC/`). Instead, `n` simply grows, and the
//! loop just keeps running for as long as it takes - there is nothing to fail, so for a
//! sufficiently large transform scale, the call now takes longer than anyone would ever wait: a
//! genuine, non-crashing hang, exactly as vello issue #1752 describes.

use std::sync::mpsc;
use std::thread;
use std::time::{Duration, Instant};

use vello_common::fearless_simd::Level;
use vello_common::flatten::{FlattenCtx, Line, fill};
use vello_common::geometry::RectU16;
use vello_common::kurbo::{Affine, BezPath};

/// An entirely ordinary quadratic Bézier curve - could be a fragment of any glyph outline or
/// piece of vector art. Nothing about this path's shape is unusual or adversarial.
fn ordinary_quad_path() -> BezPath {
    let mut path = BezPath::new();
    path.move_to((0.0, 0.0));
    path.quad_to((0.5, 1.0), (1.0, 0.0));
    path
}

/// Calls the real public `fill()` entry point with the given "zoom" scale applied as the
/// transform, on a background thread, and waits up to `timeout` for it to finish.
///
/// Returns `Some((line_count, elapsed))` if it finished in time, `None` if it timed out (the
/// background thread is simply abandoned - it keeps consuming CPU until the process exits, which
/// is fine for a one-shot repro).
fn run_fill_with_timeout(scale: f64, timeout: Duration) -> Option<(usize, Duration)> {
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let path = ordinary_quad_path();
        let mut line_buf: Vec<Line> = Vec::new();
        let mut ctx = FlattenCtx::default();
        let start = Instant::now();
        fill(
            Level::baseline(),
            path,
            Affine::scale(scale),
            &mut line_buf,
            &mut ctx,
            RectU16::new(0, 0, 65535, 65535),
        );
        // Ignore send errors: if the receiver already timed out and moved on, there is nobody
        // left to hear about this late completion, which is exactly the "still stuck" case.
        let _ = tx.send((line_buf.len(), start.elapsed()));
    });

    rx.recv_timeout(timeout).ok()
}

fn main() {
    println!("--- baseline: no zoom (scale=1) ---");
    let (n, elapsed) = run_fill_with_timeout(1.0, Duration::from_secs(30)).expect(
        "fill() at scale=1 did not complete within 30s - something is unexpectedly broken",
    );
    println!("scale=1e0: produced {n} lines in {elapsed:?}");

    println!();
    println!("--- moderate zoom levels: demonstrates elapsed time growing with scale ---");
    for exp in [10_i32, 12, 14, 16] {
        let scale = 10f64.powi(exp);
        match run_fill_with_timeout(scale, Duration::from_secs(30)) {
            Some((n, elapsed)) => println!("scale=1e{exp}: produced {n} lines in {elapsed:?}"),
            None => println!("scale=1e{exp}: TIMED OUT after 30s"),
        }
    }

    println!();
    println!(
        "--- extreme zoom (scale=1e40): a real user reaches this after enough zoom-in steps ---"
    );
    let extreme_timeout = Duration::from_secs(5);
    match run_fill_with_timeout(1e40, extreme_timeout) {
        Some((n, elapsed)) => {
            println!("scale=1e40: produced {n} lines in {elapsed:?} (did NOT hang - bug is fixed)");
        }
        None => {
            println!(
                "scale=1e40: TIMED OUT after {extreme_timeout:?} - fill() has not returned. \
                 This single call to a path-filling function, on an entirely ordinary curve, is \
                 hung: at this transform scale the internal subdivision-count estimate saturates \
                 to usize::MAX (see RESULT.md), and the flattening loop that follows would need on \
                 the order of hundreds of years of continuous CPU time to complete."
            );
            // Demonstrate this isn't a transient slowdown by trying again, briefly - it should
            // time out again, identically.
            match run_fill_with_timeout(1e40, Duration::from_secs(2)) {
                Some(_) => println!("scale=1e40 (retry): unexpectedly completed"),
                None => println!(
                    "scale=1e40 (retry): TIMED OUT again after 2s - confirmed, not a one-off."
                ),
            }
        }
    }
}
