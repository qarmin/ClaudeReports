//! Standalone reproduction for candidate 076 (GRADIENT_RAMP_MAKE_RAMP_ASSUMES_SORTED_STOPS).
//!
//! Drives the real, public `vello` API end-to-end: builds a `peniko::Gradient` with a legal
//! (peniko does not require sorted offsets) but *unsorted* `ColorStops` value, feeds it through
//! `Scene::fill` exactly as any real caller would, then resolves the scene with the real
//! `vello_encoding::Resolver` (the same resolver `vello::Renderer` uses internally before
//! uploading the gradient ramp texture to the GPU) and inspects the actual computed ramp LUT.

use peniko::color::{AlphaColor, Srgb};
use peniko::kurbo::{Affine, Rect};
use peniko::{Brush, Fill, Gradient};
use vello::Scene;
use vello::low_level::vello_encoding::Resolver;

const RED: AlphaColor<Srgb> = AlphaColor::from_rgb8(255, 0, 0);
const GREEN: AlphaColor<Srgb> = AlphaColor::from_rgb8(0, 255, 0);
const BLUE: AlphaColor<Srgb> = AlphaColor::from_rgb8(0, 0, 255);

/// Builds a scene containing a single linear-gradient-filled rect with the given stops, resolves
/// it with a fresh `Resolver` (exactly the step `vello::Renderer::render_to_texture` performs
/// before uploading the gradient ramp texture), and returns the computed 512-sample ramp.
fn ramp_for(stops: [(f32, AlphaColor<Srgb>); 3]) -> Vec<u32> {
    let gradient = Gradient::new_linear((0.0, 0.0), (100.0, 0.0)).with_stops(stops);

    let mut scene = Scene::new();
    let rect = Rect::new(0.0, 0.0, 100.0, 100.0);
    scene.fill(
        Fill::NonZero,
        Affine::IDENTITY,
        &Brush::Gradient(gradient),
        None,
        &rect,
    );

    let mut resolver = Resolver::new();
    let mut packed = Vec::new();
    let (_layout, ramps, _images) = resolver.resolve(scene.encoding(), &mut packed);
    assert_eq!(ramps.height, 1, "expected exactly one cached ramp");
    assert_eq!(ramps.width as usize, ramps.data.len());
    ramps.data.to_vec()
}

fn rgba(px: u32) -> [u8; 4] {
    px.to_ne_bytes()
}

fn main() {
    // A legal `ColorStops` value: peniko's own public gradient API (`Gradient::with_stops`,
    // `ColorStopsSource` for arrays of `(f32, AlphaColor<_>)`) never requires monotonically
    // non-decreasing offsets, and does not sort or validate this. Real-world callers can easily
    // produce stops out of order (e.g. building a `Vec<ColorStop>` from an unordered data source,
    // or from user/animation-driven offsets crossing over each other frame to frame).
    //
    // Here the middle stop (GREEN at offset 0.5) is appended *last*, after the offset-1.0 (BLUE)
    // stop, even though its own offset (0.5) is smaller.
    let unsorted = [(0.0, RED), (1.0, BLUE), (0.5, GREEN)];
    // Same three stops, offsets in non-decreasing order - this is what a "well-behaved" caller
    // would produce, and is the correct reference.
    let sorted = [(0.0, RED), (0.5, GREEN), (1.0, BLUE)];

    let unsorted_ramp = ramp_for(unsorted);
    let sorted_ramp = ramp_for(sorted);

    // Sample index for u ~= 0.5 out of 512 samples (u = i / (N_SAMPLES - 1)).
    let mid = 256;
    let unsorted_mid = rgba(unsorted_ramp[mid]);
    let sorted_mid = rgba(sorted_ramp[mid]);

    println!("sample #{mid} (u ~= {:.4})", mid as f32 / 511.0);
    println!("  unsorted-stops ramp -> rgba8 = {unsorted_mid:?}");
    println!("  sorted-stops   ramp -> rgba8 = {sorted_mid:?}");

    // With correctly sorted stops, the sample at u ~= 0.5 must be (close to) pure green, since
    // that's exactly the offset of the GREEN stop.
    assert!(
        sorted_mid[1] > 200 && sorted_mid[0] < 20 && sorted_mid[2] < 20,
        "sanity check failed: sorted-stops reference ramp is not green at u ~= 0.5 as expected: {sorted_mid:?}"
    );

    // Bug: make_ramp's cursor (`this_u`/`j`) only ever advances forward through the stops slice
    // in the order given, not in offset order. Once it has advanced past stops[1] (BLUE at
    // offset 1.0), the u <= 1.0 loop condition (`while u > this_u`) can never fire again, so
    // stops[2] (GREEN, offset 0.5) is silently skipped entirely for every sample. The whole ramp
    // is therefore a plain RED -> BLUE interpolation, and the green stop never appears anywhere
    // in the output.
    let green_found_anywhere = unsorted_ramp.iter().any(|&px| {
        let [r, g, b, _a] = rgba(px);
        g > 200 && r < 20 && b < 20
    });

    if green_found_anywhere {
        println!("BUG NOT REPRODUCED: green stop was honored even though stops were unsorted");
        std::process::exit(0);
    }

    println!(
        "BUG REPRODUCED: with unsorted ColorStops, the GREEN stop (offset 0.5) never influences \
         the ramp at all ({} of {} samples are green; expected >0), and the sample at u ~= 0.5 is \
         {unsorted_mid:?} (a red/blue blend) instead of {sorted_mid:?} (green)",
        unsorted_ramp
            .iter()
            .filter(|&&px| {
                let [r, g, b, _a] = rgba(px);
                g > 200 && r < 20 && b < 20
            })
            .count(),
        unsorted_ramp.len(),
    );
    std::process::exit(1);
}
