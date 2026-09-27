// Standalone reproduction for candidate 008.JINC_ZERO_LIMIT.
//
// Uses the project's own PUBLIC API (`resize_plane_f32`) end-to-end - no internal/`pub(crate)`
// item is touched. The bug lives in `jinc_f64`/`jinc_f32` (src/math/bessel_order_one.rs) and
// `lanczos_jinc` (src/math/lanczos.rs), which special-case `x == 0` to `0.0` instead of the
// correct mathematical limit (`J1(x)/x -> 0.5` as `x -> 0`). Those functions are only reachable
// as the kernel/window for `ResamplingFunction::{HaasnSoft, Ginseng, Lanczos2Jinc, Lanczos3Jinc,
// Lanczos4Jinc, Lanczos6Jinc}` - triggered whenever a destination sample's centre lands exactly
// on a source pixel (`dx == 0.0` exactly in `compute_weights.rs`), which happens deterministically
// for any odd integer downscale factor.
//
// Exit code convention: 0 = did not reproduce (every kernel keeps a nonzero centre tap),
// non-zero = reproduced (a kernel's own centre tap silently vanished to 0.0).

use pic_scale_safe::{resize_plane_f32, ImageSize, ResamplingFunction};

const PEAK_THRESHOLD: f32 = 1e-3;

fn check(name: &str, dst: &[f32], center_index: usize, failures: &mut Vec<String>) {
    let v = dst[center_index];
    println!("{name}: dst[{center_index}] = {v}  (full: {dst:?})");
    if v.abs() < PEAK_THRESHOLD {
        failures.push(format!(
            "{name}: centre tap dst[{center_index}] = {v} (expected a clearly nonzero peak, \
             not silently zeroed)"
        ));
    }
}

fn main() {
    let mut failures: Vec<String> = Vec::new();

    // Case A: scale = in_w/out_w = 3 (odd), so center = (i+0.5)*3 - 0.5 = 3*i + 1 is an integer
    // for every destination index i - dx == 0.0 exactly at k = 3*i + 1. Put the only nonzero
    // source sample exactly there for i = 4 (k = 13).
    let in_w = 27usize;
    let out_w = 9usize;
    let mut src_a = vec![0f32; in_w];
    src_a[13] = 1.0;

    for f in [
        ResamplingFunction::HaasnSoft,
        ResamplingFunction::Ginseng,
        ResamplingFunction::Lanczos3Jinc,
    ] {
        let dst = resize_plane_f32(
            &src_a,
            ImageSize::new(in_w, 1),
            ImageSize::new(out_w, 1),
            f,
        )
        .unwrap();
        check(&format!("{f:?} (scale=3)"), &dst, 4, &mut failures);
    }

    // Case B: scale = 5 (also odd), center = 5*i + 2 is an integer - dx == 0.0 exactly at
    // k = 5*i + 2. Exercises Lanczos2Jinc/4Jinc/6Jinc, which reach `lanczos_jinc`'s own x==0
    // special case directly (no window involved), a distinct call path from case A's HaasnSoft/
    // Ginseng (which reach `jinc_f64`/`jinc_f32`'s x==0 special case through `T::jinc()` used
    // directly as the kernel/window).
    let in_w2 = 45usize;
    let out_w2 = 9usize;
    let mut src_b = vec![0f32; in_w2];
    src_b[22] = 1.0;

    for f in [
        ResamplingFunction::Lanczos2Jinc,
        ResamplingFunction::Lanczos4Jinc,
        ResamplingFunction::Lanczos6Jinc,
    ] {
        let dst = resize_plane_f32(
            &src_b,
            ImageSize::new(in_w2, 1),
            ImageSize::new(out_w2, 1),
            f,
        )
        .unwrap();
        check(&format!("{f:?} (scale=5)"), &dst, 4, &mut failures);
    }

    if failures.is_empty() {
        println!("did not reproduce - every kernel's centre tap stayed nonzero");
        std::process::exit(0);
    } else {
        eprintln!("REPRODUCED - {} check(s) failed:", failures.len());
        for f in &failures {
            eprintln!("  - {f}");
        }
        std::process::exit(1);
    }
}
