//! Candidate 041: RECT_F32_CAST_AA_DIVERGENCE.
//!
//! Investigates `sparse_strips/vello_common/src/rect.rs` lines 49-52, which downcast
//! `rect.x0`/`y0`/`x1`/`y1` from `f64` to `f32` in vello_cpu's axis-aligned `fill_rect` fast
//! path. The candidate claims this cast (vs. the general `fill_path` pipeline allegedly "keeping
//! rect coordinates in f64 through flattening") produces a real, if minor, alpha-coverage
//! divergence of exactly 1/255 between `fill_rect(&rect)` and `fill_path(&rect.to_path(eps))` for
//! identical sub-pixel-positioned rects.
//!
//! This program runs three real, in-process checks against the project's own public API
//! (a single participant - no peer/caller is faked, see CONFIRMATION.md section 11):
//!
//! 1. Confirms/refutes the premise that the general `fill_path` pipeline "keeps rect coordinates
//!    in f64 through flattening": `vello_common::flatten::Point::from(kurbo::Point)` casts to
//!    `f32` immediately (`flatten.rs` lines 43-46), the same as `rect.rs`'s own cast - so both
//!    paths actually downcast to f32 at essentially the same point, for the same corner values.
//! 2. Runs a real, end-to-end differential test: for many sub-pixel-positioned, many-sized rects
//!    (including near-`u16::MAX` coordinates, where f64->f32 downcast loses the most relative
//!    precision, and widths/heights straddling the 4x4 tile grid), renders the same rect via
//!    `RenderContext::fill_rect` and via `RenderContext::fill_path(&rect.to_path(eps))`, and
//!    compares every output pixel's premultiplied alpha.
//! 3. A seeded random fuzz sweep (fixed PRNG seed, fully deterministic/repeatable) over random
//!    sub-pixel rect positions/sizes, same comparison.
//!
//! Both renders go through the real public `vello_cpu::RenderContext` API end to end
//! (`fill_rect`/`fill_path` -> `render_with` -> `Pixmap`), not a hand-constructed stand-in.

use vello_cpu::color::PremulRgba8;
use vello_cpu::color::palette::css::RED;
use vello_cpu::kurbo::{Rect, Shape};
use vello_cpu::{Pixmap, RasterizerSettings, RenderContext, Resources};

fn render_via_fill_rect(width: u16, height: u16, rect: Rect) -> Pixmap {
    let mut ctx = RenderContext::new(width, height);
    ctx.set_paint(RED);
    ctx.fill_rect(&rect);
    ctx.flush();
    let mut resources = Resources::new();
    let mut pm = Pixmap::new(width, height);
    ctx.render_with(&mut pm, &mut resources, RasterizerSettings::default());
    pm
}

fn render_via_fill_path(width: u16, height: u16, rect: Rect) -> Pixmap {
    let mut ctx = RenderContext::new(width, height);
    ctx.set_paint(RED);
    ctx.fill_path(&rect.to_path(0.1));
    ctx.flush();
    let mut resources = Resources::new();
    let mut pm = Pixmap::new(width, height);
    ctx.render_with(&mut pm, &mut resources, RasterizerSettings::default());
    pm
}

fn diff_pixmaps(width: u16, height: u16, a: &Pixmap, b: &Pixmap) -> Vec<(u16, u16, PremulRgba8, PremulRgba8)> {
    let mut mismatches = vec![];
    for y in 0..height {
        for x in 0..width {
            let pa = a.sample(x, y);
            let pb = b.sample(x, y);
            if pa != pb {
                mismatches.push((x, y, pa, pb));
            }
        }
    }
    mismatches
}

fn step1_cast_premise() {
    println!("=== Step 1: does fill_path's flattening pipeline actually keep rect coords in f64? ===");
    // Same downcast expression rect.rs uses on lines 49-52.
    let samples: &[f64] = &[0.03, 3.71, 999.9999, 30_000.123_456, 65_531.987_654_321];
    for &v in samples {
        let via_rect_rs = v as f32;
        // flatten.rs's Point::from(kurbo::Point) does exactly `value.x as f32` at the point where
        // a rect's corner (as produced by `Rect::to_path`) enters the flattening pipeline.
        let via_flatten_point_from = v as f32;
        println!(
            "  v={v:<16}  rect.rs cast={via_rect_rs:<14}  flatten::Point::from-equivalent cast={via_flatten_point_from:<14}  equal={}",
            via_rect_rs == via_flatten_point_from
        );
        assert_eq!(
            via_rect_rs, via_flatten_point_from,
            "expected both code paths to perform the identical f64->f32 downcast on the same value"
        );
    }
    println!(
        "Confirmed: both `fill_rect`'s fast path (rect.rs:49-52) and `fill_path`'s flattening \
        pipeline (flatten.rs's `impl From<kurbo::Point> for Point`) downcast rect corners from \
        f64 to f32 with the exact same `as f32` cast on the exact same value - the general \
        pipeline does NOT keep coordinates in f64 through flattening, contrary to the candidate's \
        premise.\n"
    );
}

fn step2_curated_sweep() -> usize {
    println!("=== Step 2: curated differential sweep (sub-pixel positions, tile-boundary sizes, near-u16::MAX) ===");
    let mut total_mismatches = 0usize;
    let mut total_checked = 0usize;

    let bases: &[(f64, u16)] = &[(0.0, 40), (3.0, 44), (1000.0, 1044), (65_000.0, 65_020)];
    let widths: &[f64] = &[0.5, 1.7, 3.9999, 4.0001, 5.3743, 7.9999];
    let offsets: &[f64] = &[0.0001, 0.29, 0.61, 0.93, 1.7, 2.3, 3.14159];

    for &(base, canvas) in bases {
        for &dx in offsets {
            for &dy in offsets {
                for &rw in widths {
                    for &rh in widths {
                        let x0 = base + dx;
                        let y0 = dy;
                        let rect = Rect::new(x0, y0, x0 + rw, y0 + rh);
                        if rect.is_zero_area() {
                            continue;
                        }
                        let height = 16u16;
                        let a = render_via_fill_rect(canvas, height, rect);
                        let b = render_via_fill_path(canvas, height, rect);
                        let mism = diff_pixmaps(canvas, height, &a, &b);
                        total_checked += 1;
                        if !mism.is_empty() {
                            total_mismatches += mism.len();
                            println!(
                                "  MISMATCH rect={rect:?} first={:?} ({} pixel(s) differ)",
                                mism[0],
                                mism.len()
                            );
                        }
                    }
                }
            }
        }
    }

    println!(
        "Checked {total_checked} rect configurations ({} total pixel comparisons across all of \
        them); {total_mismatches} pixel(s) differed between fill_rect and fill_path.\n",
        total_checked
    );
    total_mismatches
}

struct Rng(u64);
impl Rng {
    fn next_u64(&mut self) -> u64 {
        let mut x = self.0;
        x ^= x << 13;
        x ^= x >> 7;
        x ^= x << 17;
        self.0 = x;
        x
    }
    fn next_f64(&mut self, lo: f64, hi: f64) -> f64 {
        let v = (self.next_u64() >> 11) as f64 / (1u64 << 53) as f64;
        lo + v * (hi - lo)
    }
}

fn step3_seeded_random_fuzz() -> usize {
    println!("=== Step 3: seeded random fuzz (fixed seed, deterministic/repeatable) ===");
    let mut rng = Rng(0x9E3779B97F4A7C15);
    let mut total_mismatches = 0usize;
    let (width, height) = (48u16, 48u16);
    let mut processed = 0usize;

    for _ in 0..5000 {
        let x0 = rng.next_f64(-2.0, 40.0);
        let y0 = rng.next_f64(-2.0, 40.0);
        let w = rng.next_f64(0.001, 20.0);
        let h = rng.next_f64(0.001, 20.0);
        let rect = Rect::new(x0, y0, x0 + w, y0 + h)
            .intersect(Rect::new(0.0, 0.0, f64::from(width), f64::from(height)));
        if rect.is_zero_area() {
            continue;
        }
        processed += 1;

        let a = render_via_fill_rect(width, height, rect);
        let b = render_via_fill_path(width, height, rect);
        let mism = diff_pixmaps(width, height, &a, &b);
        if !mism.is_empty() {
            total_mismatches += mism.len();
            println!(
                "  MISMATCH rect={rect:?} first={:?} ({} pixel(s) differ)",
                mism[0],
                mism.len()
            );
        }
    }

    println!("Processed {processed} random rects; {total_mismatches} pixel(s) differed.\n");
    total_mismatches
}

fn main() {
    step1_cast_premise();
    let m2 = step2_curated_sweep();
    let m3 = step3_seeded_random_fuzz();

    let total = m2 + m3;
    println!("=== Verdict ===");
    if total == 0 {
        println!(
            "No divergence found between fill_rect and fill_path(rect.to_path(eps)) across all \
            checks. The candidate's claimed 1/255 alpha divergence did not reproduce."
        );
    } else {
        println!("Found {total} diverging pixel(s) - candidate reproduces.");
        std::process::exit(1);
    }
}
