// Standalone reproduction for candidate 010.GENERATE_WEIGHTS_ZERO_SUM_FALLBACK.
//
// Uses the project's own PUBLIC API (`resize_plane_f32`) end-to-end - no internal/`pub(crate)`
// item is touched. The bug lives in the general (non-area) branch of `generate_weights` in
// `src/compute_weights.rs`, which only writes normalized weights `if weights_sum != 0.0` and has
// no fallback for a row whose whole kernel window evaluates to exactly 0.0 (unlike the sibling
// area-filter branch a few lines below, which explicitly falls back to
// `weights[filter_position] = 1.0`). Such a degenerate all-zero-sum row is left as its
// zero-initialized default, so every destination sample using that row silently becomes 0.0 -
// even though there is a real, nonzero source pixel available.
//
// `ResamplingFunction::HaasnSoft`/`Lanczos6Jinc` with a 1-pixel-wide source hit this reliably:
// with `in_size == 1`, `filter_scale_cutoff == out_size` (since these kernels are "resizable" and
// `scale = in_size/out_size` is tiny, but `filter_scale_cutoff = scale.max(1.0)` clamps it back up
// only when scale >= 1 - for a large upscale the *window* half-width in kernel-space still grows
// with `out_size`, and many destination rows land on kernel x-values where the whole window
// evaluates to 0.0), producing rows whose only source tap has a kernel value of exactly 0.0.
//
// Exit code convention: 0 = did not reproduce (every destination sample kept a nonzero value
// matching the single source sample), non-zero = reproduced (some destination samples were
// silently zeroed even though the source pixel was nonzero).

use pic_scale_safe::{resize_plane_f32, ImageSize, ResamplingFunction};

fn main() {
    let mut failures: Vec<String> = Vec::new();

    // Case A: a single nonzero source pixel, upscaled 100x with HaasnSoft - every output sample
    // should reproduce the single source value (there is nothing else to interpolate from).
    let src_a = vec![0.7f32];
    let dst_a = resize_plane_f32(
        &src_a,
        ImageSize::new(1, 1),
        ImageSize::new(100, 1),
        ResamplingFunction::HaasnSoft,
    )
    .unwrap();
    println!("HaasnSoft 1->100: {dst_a:?}");
    let zero_count_a = dst_a.iter().filter(|&&v| v == 0.0).count();
    if zero_count_a > 0 {
        failures.push(format!(
            "HaasnSoft 1x1->100x1: {zero_count_a} of {} destination samples were silently \
             zeroed despite a nonzero (0.7) source pixel",
            dst_a.len()
        ));
    }

    // Case B: same bug class, different scale/size - HaasnSoft, 3->10, all-nonzero source row
    // (so any zeroed output can only come from a degenerate zero-sum kernel row, not from an
    // actually-zero source sample).
    let src_b = vec![1.0f32, 2.0f32, 3.0f32];
    let dst_b = resize_plane_f32(
        &src_b,
        ImageSize::new(3, 1),
        ImageSize::new(10, 1),
        ResamplingFunction::HaasnSoft,
    )
    .unwrap();
    println!("HaasnSoft 3->10: {dst_b:?}");
    let zero_count_b = dst_b.iter().filter(|&&v| v == 0.0).count();
    if zero_count_b > 0 {
        failures.push(format!(
            "HaasnSoft 3x1->10x1: {zero_count_b} of {} destination samples were silently \
             zeroed despite an all-nonzero source row",
            dst_b.len()
        ));
    }

    if failures.is_empty() {
        println!("did not reproduce - every destination sample stayed nonzero");
        std::process::exit(0);
    } else {
        eprintln!("REPRODUCED - {} check(s) failed:", failures.len());
        for f in &failures {
            eprintln!("  - {f}");
        }
        std::process::exit(1);
    }
}
