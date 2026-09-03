// Repro for candidate 036: exact float equality gates font-hinting uniform-scale detection
// in vello_encoding/src/resolve.rs (classic `vello` crate's glyph-run resolution).
//
// This drives the REAL public entry point an application uses (vello::Scene::draw_glyphs(..)
// .transform(..).hint(true).draw(..)), builds the glyph run's transform via genuine matrix
// composition (three rotations that mathematically sum to zero, sandwiching a uniform scale -
// exactly the shape a real transform hierarchy produces), then resolves the scene with
// vello_encoding::Resolver::resolve - the same call vello::Renderer makes internally when
// rendering - and inspects the packed output's transform stream (also public API) to check
// whether the y-translation was hint-rounded to the pixel grid.

use std::sync::Arc;

use kurbo::Affine;
use vello::Glyph;
use vello::Scene;
use vello::peniko::{Blob, Fill, FontData};
use vello_encoding::Resolver;
use skrifa::MetadataProvider;

const ROBOTO_FONT: &[u8] = include_bytes!("../assets/Roboto-Regular.ttf");

fn main() {
    let font = FontData::new(Blob::new(Arc::new(ROBOTO_FONT)), 0);

    // A transform built via matrix composition that is mathematically a pure uniform scale
    // (three rotations summing to exactly zero, sandwiching a scale by 3.0) - the kind of
    // transform a real hierarchy (parent rotate * content scale * inverse-parent rotate) would
    // produce, not a literal `Affine::scale(3.0)` call.
    let t1 = 0.123_456_7_f64;
    let t2 = 0.765_432_1_f64;
    let t3 = -(t1 + t2);
    let composed = Affine::rotate(t1) * Affine::rotate(t2) * Affine::scale(3.0) * Affine::rotate(t3);
    let coeffs = composed.as_coeffs();
    let coeffs32: [f32; 4] = [
        coeffs[0] as f32,
        coeffs[1] as f32,
        coeffs[2] as f32,
        coeffs[3] as f32,
    ];
    println!("composed matrix (f32): {coeffs32:?}");
    println!(
        "exact-equality uniform-scale check would pass: {}",
        coeffs32[0] == coeffs32[3] && coeffs32[1] == 0.0 && coeffs32[2] == 0.0
    );
    assert!(
        !(coeffs32[0] == coeffs32[3] && coeffs32[1] == 0.0 && coeffs32[2] == 0.0),
        "setup invariant broken: this composed transform must NOT satisfy exact equality \
         (matrix[1]/matrix[2] must be nonzero after casting to f32) for this repro to be valid"
    );

    // A fractional y translation, chosen so that whether hinting actually rounds it to the pixel
    // grid is directly observable in the resolved output.
    let transform = Affine::translate((100.0, 100.37)) * composed;

    let mut scene = Scene::new();
    let font_ref =
        skrifa::FontRef::from_index(font.data.as_ref(), font.index).expect("valid font");
    let gid = font_ref.charmap().map('A').expect("glyph for 'A' exists");

    scene
        .draw_glyphs(&font)
        .font_size(32.0)
        .transform(transform)
        .hint(true) // hinting explicitly requested by the caller
        .draw(
            Fill::NonZero,
            [Glyph {
                id: gid.to_u32(),
                x: 0.0,
                y: 0.0,
            }]
            .into_iter(),
        );

    // This is the exact same resolution step `vello::Renderer` performs internally when
    // rendering the scene - real end-to-end trigger path, not a hand-built stand-in.
    let mut resolver = Resolver::new();
    let mut packed = Vec::new();
    let (layout, _ramps, _images) = resolver.resolve(&scene.encoding(), &mut packed);

    let transforms = layout.transforms(&packed);
    // The glyph's own transform is the first entry written for this run; the paint transform
    // follows it (see resolve.rs's transform-stream packing loop).
    let glyph_transform = transforms[0];
    println!("resolved glyph transform: {glyph_transform:?}");

    let y = glyph_transform.translation[1];
    let was_hint_rounded = y == y.round();
    println!("y-translation = {y}, hint-rounded to pixel grid = {was_hint_rounded}");

    if was_hint_rounded {
        println!("did not reproduce - hinting was correctly applied (y snapped to pixel grid)");
    } else {
        eprintln!(
            "REPRODUCED - a transform that is mathematically a pure uniform scale (produced by \
             genuine matrix composition: rotate * rotate * scale * rotate, angles summing to \
             zero) failed the exact-equality check, silently disabling hinting: y-translation \
             {y} was left un-rounded instead of snapping to the pixel grid at {}",
            y.round()
        );
        std::process::exit(1);
    }
}
