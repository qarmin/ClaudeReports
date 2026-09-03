// Variant of the main repro (candidate 092) using a different boundary (n=0) and a different
// entry point (`generate_filled_path` instead of `generate_filled_rect_fast`), to confirm the fix
// isn't narrowly tailored to the exact original input (CONFIRMATION.md section 7).

use vello_common::fearless_simd::Level;
use vello_common::kurbo::{Affine, Rect, Shape};
use vello_common::peniko::Fill;
use vello_common::strip_generator::{GenerationMode, StripGenerator, StripStorage};

fn main() {
    let mut generator = StripGenerator::new(200, 200, Level::baseline());
    let mut storage = StripStorage::default();

    // n=0 boundary: every frame should discard *everything* from the previous frame.
    storage.set_generation_mode(GenerationMode::ReplaceAfter(0));

    let rect = Rect::new(5.0, 5.0, 40.0, 40.0);
    const FRAMES: usize = 300;
    for _ in 0..FRAMES {
        generator.generate_filled_path(
            rect.to_path(0.1),
            Fill::NonZero,
            Affine::IDENTITY,
            None,
            &mut storage,
            None,
        );
    }

    println!(
        "n=0, generate_filled_path, after {FRAMES} frames: strips={} alphas={}",
        storage.strips.len(),
        storage.alphas.len()
    );

    // With n=0 every frame fully replaces state, so both strips and alphas should stay bounded
    // by a single frame's worth of data, independent of FRAMES.
    if storage.alphas.len() > 20_000 {
        println!(
            "BUG STILL PRESENT: alphas grew unboundedly ({} bytes after {FRAMES} frames)",
            storage.alphas.len()
        );
        std::process::exit(1);
    }
    println!("alphas stayed bounded - fix holds for n=0 / generate_filled_path too.");
}
