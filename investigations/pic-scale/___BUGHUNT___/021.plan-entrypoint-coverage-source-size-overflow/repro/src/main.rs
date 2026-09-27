// Standalone reproduction, real public API, single participant (a caller passing raw
// width/height integers straight to `pic_scale::ImageSize`/`Scaler::plan_*_resampling*` - no
// second participant to fake, this is the genuine external input itself).
//
// Follow-up to candidate 010: that candidate confirmed and fixed an unvalidated-`source_size`
// overflow panic in `src/math/weights.rs::generate_weights`, but only drove it through
// `Scaler::plan_rgba_resampling` (Lanczos3, u8). This repro drives the *same* overflow-adjacent
// `source_size`/`destination_size` pair through every other public `plan_*_resampling*` entry
// point in `src/scaler.rs` (all pixel-format/bit-depth/AlphaPlanner/DefaultPlanner/f32-workload-
// strategy combinations, plus AR30/RA30) to check whether any of them takes a subtly different
// path that the same fix doesn't cover.
//
// Exit code convention: 0 = none of the tested entry points reproduced (bug class absent/fixed),
// non-zero = at least one entry point panicked instead of returning `Result::Err` (count printed
// on stderr).

use pic_scale::{Ar30ByteOrder, ImageSize, ResamplingFunction, Scaler, WorkloadStrategy};
use std::panic::{self, AssertUnwindSafe};

fn src() -> ImageSize {
    ImageSize::new(1usize << 62, 1)
}
fn dst() -> ImageSize {
    ImageSize::new(2, 1)
}

fn run(name: &str, f: impl FnOnce() + panic::UnwindSafe) -> bool {
    match panic::catch_unwind(AssertUnwindSafe(f)) {
        Ok(()) => {
            println!("{name}: did not reproduce (no panic)");
            false
        }
        Err(e) => {
            let msg = if let Some(s) = e.downcast_ref::<&str>() {
                s.to_string()
            } else if let Some(s) = e.downcast_ref::<String>() {
                s.clone()
            } else {
                "<non-string panic payload>".to_string()
            };
            println!("{name}: REPRODUCED - panicked instead of returning Result::Err: {msg}");
            true
        }
    }
}

fn main() {
    panic::set_hook(Box::new(|_info| {}));

    let scaler = Scaler::new(ResamplingFunction::Lanczos3);
    let mut scaler_q = Scaler::new(ResamplingFunction::Lanczos3);
    scaler_q.set_workload_strategy(WorkloadStrategy::PreferQuality);
    let mut scaler_s = Scaler::new(ResamplingFunction::Lanczos3);
    scaler_s.set_workload_strategy(WorkloadStrategy::PreferSpeed);
    let scaler_nearest = Scaler::new(ResamplingFunction::Nearest);

    let mut panicked = 0usize;
    let mut total = 0usize;

    macro_rules! check {
        ($name:expr, $call:expr) => {{
            total += 1;
            if run($name, || {
                let _ = $call;
            }) {
                panicked += 1;
            }
        }};
    }

    check!("plan_planar_resampling (u8,N=1)", scaler.plan_planar_resampling(src(), dst()));
    check!(
        "plan_gray_alpha_resampling no-premult (u8,N=2)",
        scaler.plan_gray_alpha_resampling(src(), dst(), false)
    );
    check!(
        "plan_gray_alpha_resampling premult (u8,N=2, AlphaPlanner)",
        scaler.plan_gray_alpha_resampling(src(), dst(), true)
    );
    check!("plan_cbcr_resampling (u8,N=2)", scaler.plan_cbcr_resampling(src(), dst()));
    check!("plan_rgb_resampling (u8,N=3)", scaler.plan_rgb_resampling(src(), dst()));
    check!(
        "plan_rgba_resampling no-premult (u8,N=4)",
        scaler.plan_rgba_resampling(src(), dst(), false)
    );
    check!(
        "plan_rgba_resampling premult (u8,N=4, AlphaPlanner)",
        scaler.plan_rgba_resampling(src(), dst(), true)
    );

    check!(
        "plan_planar_resampling16 (u16,N=1,bd=16)",
        scaler.plan_planar_resampling16(src(), dst(), 16)
    );
    check!(
        "plan_planar_resampling_s16 (i16,N=1,bd=16)",
        scaler.plan_planar_resampling_s16(src(), dst(), 16)
    );
    check!(
        "plan_cbcr_resampling16 (u16,N=2,bd=16)",
        scaler.plan_cbcr_resampling16(src(), dst(), 16)
    );
    check!(
        "plan_gray_alpha_resampling16 no-premult (u16,N=2)",
        scaler.plan_gray_alpha_resampling16(src(), dst(), false, 16)
    );
    check!(
        "plan_gray_alpha_resampling16 premult (u16,N=2, AlphaPlanner)",
        scaler.plan_gray_alpha_resampling16(src(), dst(), true, 16)
    );
    check!(
        "plan_rgb_resampling16 (u16,N=3,bd=16)",
        scaler.plan_rgb_resampling16(src(), dst(), 16)
    );
    check!(
        "plan_rgba_resampling16 no-premult (u16,N=4)",
        scaler.plan_rgba_resampling16(src(), dst(), false, 16)
    );
    check!(
        "plan_rgba_resampling16 premult (u16,N=4, AlphaPlanner)",
        scaler.plan_rgba_resampling16(src(), dst(), true, 16)
    );

    check!(
        "plan_planar_resampling_f32 PreferQuality (f32,W=f64)",
        scaler_q.plan_planar_resampling_f32(src(), dst())
    );
    check!(
        "plan_planar_resampling_f32 PreferSpeed (f32,W=f32)",
        scaler_s.plan_planar_resampling_f32(src(), dst())
    );
    check!(
        "plan_cbcr_resampling_f32 PreferQuality (f32,N=2,W=f64)",
        scaler_q.plan_cbcr_resampling_f32(src(), dst())
    );
    check!(
        "plan_gray_alpha_resampling_f32 premult PreferQuality (f32,N=2, AlphaPlanner)",
        scaler_q.plan_gray_alpha_resampling_f32(src(), dst(), true)
    );
    check!(
        "plan_rgb_resampling_f32 PreferQuality (f32,N=3)",
        scaler_q.plan_rgb_resampling_f32(src(), dst())
    );
    check!(
        "plan_rgba_resampling_f32 premult PreferQuality (f32,N=4, AlphaPlanner)",
        scaler_q.plan_rgba_resampling_f32(src(), dst(), true)
    );

    check!(
        "plan_ar30_resampling Host (u8,N=4, via rgb16)",
        scaler.plan_ar30_resampling(src(), dst(), Ar30ByteOrder::Host)
    );
    check!(
        "plan_ra30_resampling Network (u8,N=4, via rgb16)",
        scaler.plan_ra30_resampling(src(), dst(), Ar30ByteOrder::Network)
    );
    check!(
        "plan_ar30_resampling Host, Nearest (bypasses generate_weights, expected to never reproduce)",
        scaler_nearest.plan_ar30_resampling(src(), dst(), Ar30ByteOrder::Host)
    );

    eprintln!("{panicked}/{total} entry points panicked");
    if panicked > 0 {
        std::process::exit(1);
    }
}
