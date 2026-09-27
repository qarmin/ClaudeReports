// Reproduction for candidate 092: `GenerationMode::ReplaceAfter(n)` truncates
// `strip_storage.strips` to `[0..n]` in `render_with_clip`, but never truncates
// `strip_storage.alphas` to match. Repeated ReplaceAfter-based partial redraws (the API's
// evident purpose - incremental/retained-mode scene updates) grow `alphas.len()`/`capacity()`
// unboundedly across frames even though `strips.len()` stays bounded and the strip metadata
// that referenced the discarded alpha region is gone and never read again.
//
// This drives the real public API (`StripGenerator::generate_filled_rect_fast`,
// `StripStorage::set_generation_mode`) exactly as a retained-mode caller redrawing one dirty
// rect per frame would use it - there is no second participant to fake here, this is a single
// process calling a public API in a loop.

use vello_common::fearless_simd::Level;
use vello_common::kurbo::Rect;
use vello_common::strip_generator::{GenerationMode, StripGenerator, StripStorage};

fn main() {
    let width = 2000_u16;
    let height = 2000_u16;
    let mut generator = StripGenerator::new(width, height, Level::baseline());
    let mut storage = StripStorage::default();

    // Frame 0: draw a "static background" that a retained-mode scene keeps around
    // (e.g. UI chrome that never changes) using the default `Replace` mode.
    let background = Rect::new(0.0, 0.0, 2000.0, 2000.0);
    generator.generate_filled_rect_fast(&background, &mut storage, None);

    let static_strip_count = storage.strips.len();
    let static_alpha_count = storage.alphas.len();
    println!(
        "after static background: strips={static_strip_count} alphas={static_alpha_count} alphas.capacity={}",
        storage.alphas.capacity()
    );
    assert!(static_strip_count > 0);
    assert!(static_alpha_count > 0);

    // From here on, every frame redraws only a small "dirty rect" on top of the static
    // background, using `ReplaceAfter(static_strip_count)` - exactly the incremental-redraw
    // pattern this API mode exists for: keep strips [0..n] (the static background), replace
    // everything after that with this frame's freshly generated strips.
    storage.set_generation_mode(GenerationMode::ReplaceAfter(static_strip_count));

    let dirty_rect = Rect::new(10.0, 10.0, 50.0, 50.0);
    const FRAMES: usize = 500;
    for _ in 0..FRAMES {
        generator.generate_filled_rect_fast(&dirty_rect, &mut storage, None);
    }

    let final_strip_count = storage.strips.len();
    let final_alpha_count = storage.alphas.len();
    println!(
        "after {FRAMES} incremental redraws: strips={final_strip_count} alphas={final_alpha_count} alphas.capacity={}",
        storage.alphas.capacity()
    );

    // `strips` stays bounded: static background strips + this one frame's dirty-rect strips.
    // `alphas` should behave the same way (bounded by the same live strips), but instead grows
    // by one dirty-rect's worth of alpha bytes every single frame, since the alphas written for
    // every discarded prior frame are never reclaimed.
    let alpha_growth_per_frame = (final_alpha_count - static_alpha_count) as f64 / FRAMES as f64;

    println!("strips bounded: {final_strip_count} (not growing with FRAMES)");
    println!(
        "alphas grew by ~{alpha_growth_per_frame:.1} bytes/frame over {FRAMES} frames -> unbounded growth"
    );

    assert!(
        final_strip_count < 10 * static_strip_count,
        "strips grew unexpectedly: {final_strip_count}"
    );

    // If the bug is present, alphas grows roughly linearly with FRAMES (one dirty-rect's worth
    // of orphaned alpha bytes retained per discarded frame). If it were fixed (alphas truncated
    // in lockstep with strips), alpha_count after N frames would be close to a single frame's
    // worth, independent of N.
    if final_alpha_count > static_alpha_count + 50 * FRAMES {
        println!(
            "BUG CONFIRMED: alphas grew unboundedly across {FRAMES} ReplaceAfter frames \
             ({final_alpha_count} bytes) while strips stayed bounded ({final_strip_count})."
        );
        std::process::exit(1);
    } else {
        println!("alphas stayed bounded across frames - bug not present in this build.");
    }
}
