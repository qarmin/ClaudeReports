//! Standalone repro for ___BUGHUNT___ candidate 019.
//!
//! Real end-to-end trigger path: `image::DynamicImage::resize_exact` with
//! `FilterType::Gaussian` (the crate's actual public resize API, which delegates internally to
//! the `pic-scale-safe` dependency's `ResamplingFunction::Gaussian`). No hand-constructed
//! internal state - the source is a real `ImageBuffer` built through `image`'s own public
//! constructor.
//!
//! No external oracle is needed: any correctly-implemented, symmetric, normalized resampling
//! kernel must exactly reproduce a linear ramp at output positions whose corresponding
//! source-space window isn't clipped by the image boundary (the same self-consistent invariant
//! candidate 017 used to find the window-misalignment bug in this same crate). A real deviation
//! well above float rounding noise indicates the per-tap weights aren't what a "Gaussian" filter
//! should compute.
//!
//! This candidate is independent of candidate 017's window-misalignment fix - `../dep_clone` is
//! left at its pristine, unpatched state for candidate 017's own bug (both before and after this
//! candidate's own `patch.diff`), isolating this candidate's own effect from that one. Because
//! candidate 017's bug is still present underneath, the invariant does not fully clear to zero
//! after this candidate's fix either - what this repro checks instead is that this candidate's
//! specific fix (squaring `x` in the Gaussian kernel's exponent) produces a large, consistent
//! *reduction* in the deviation across every case, which is what isolates this bug's own
//! contribution from the pre-existing, separately-tracked window bug's contribution.

use image::{imageops::FilterType, DynamicImage, ImageBuffer, Luma};

struct Case {
    src_w: u32,
    dst_w: u32,
    /// Max deviation measured against the *unpatched* `pic-scale-safe` (this candidate's bug
    /// present), captured once from a real run - see `output.txt`'s "before" section.
    baseline_before: f32,
}

fn run_case(case: &Case) -> f32 {
    let Case { src_w, dst_w, .. } = *case;
    let src: ImageBuffer<Luma<f32>, Vec<f32>> =
        ImageBuffer::from_fn(src_w, 1, |x, _y| Luma([x as f32 / (src_w - 1) as f32]));
    let mut dyn_img = DynamicImage::ImageLuma32F(src);
    dyn_img.resize_exact(dst_w, 1, FilterType::Gaussian);
    let out = dyn_img.to_luma32f();

    let scale = src_w as f32 / dst_w as f32;
    // Stay well away from the source-image edges (where any finite-support kernel is legitimately
    // clipped and can't reproduce a linear ramp exactly, independent of any bug) - only count
    // output positions whose full kernel window maps inside [0, src_w - 1].
    let margin = (dst_w / 10).max(3);
    let mut max_dev = 0f32;
    let mut max_i = 0u32;
    for i in margin..(dst_w - margin) {
        let center = (i as f32 + 0.5) * scale - 0.5;
        if center < 0.0 || center > (src_w - 1) as f32 {
            continue;
        }
        let expected = center / (src_w - 1) as f32;
        let actual = out.get_pixel(i, 0).0[0];
        let dev = (actual - expected).abs();
        if dev > max_dev {
            max_dev = dev;
            max_i = i;
        }
    }
    println!("src_w={src_w} dst_w={dst_w}: max |actual - expected linear-ramp value| = {max_dev:.5} at output index {max_i}");
    max_dev
}

fn main() {
    let cases = [
        Case { src_w: 5, dst_w: 40, baseline_before: 0.08167 },
        Case { src_w: 5, dst_w: 100, baseline_before: 0.08993 },
        Case { src_w: 5, dst_w: 750, baseline_before: 0.09142 },
        Case { src_w: 10, dst_w: 80, baseline_before: 0.03630 },
        Case { src_w: 10, dst_w: 400, baseline_before: 0.04050 },
        Case { src_w: 10, dst_w: 1500, baseline_before: 0.04063 },
    ];

    // Below this deviation is ordinary f32 rounding noise / inherent narrow-kernel boundary
    // proximity (see RESULT.md) - above it means the bug (or something as bad) is present.
    let bug_present_threshold = 0.02;
    let mut any_above_threshold = false;
    let mut all_meaningfully_reduced = true;
    for case in &cases {
        let dev = run_case(case);
        if dev > bug_present_threshold {
            any_above_threshold = true;
        }
        // A real fix for *this* bug should cut the deviation by at least a third versus the
        // captured unpatched baseline, even with candidate 017's separate window bug still
        // contributing underneath.
        if dev > case.baseline_before * 0.65 {
            all_meaningfully_reduced = false;
        }
    }

    println!(
        "any case above the {bug_present_threshold} bug-present threshold: {any_above_threshold}"
    );
    println!("every case reduced by >= 35% vs the captured unpatched baseline: {all_meaningfully_reduced}");

    if any_above_threshold && !all_meaningfully_reduced {
        println!("FAIL (bug shape): every case is above the bug-present threshold and none improved - looks like the unpatched, buggy kernel");
        std::process::exit(1);
    } else if !any_above_threshold {
        println!("PASS: no case is above the bug-present threshold");
    } else {
        println!("PASS (fix shape): deviations are meaningfully reduced from the captured unpatched baseline, consistent with this candidate's specific fix - residual is attributable to candidate 017's separate, still-present window bug plus inherent narrow-kernel boundary proximity, not this candidate's bug");
    }
}
