// Sanity check that the tolerance-based fix does not over-broaden: a genuinely non-uniform
// scale / skew transform must still have hinting disabled.
use std::sync::Arc;

use kurbo::Affine;
use vello::Glyph;
use vello::Scene;
use vello::peniko::{Blob, Fill, FontData};
use vello_encoding::Resolver;
use skrifa::MetadataProvider;

const ROBOTO_FONT: &[u8] = include_bytes!("../../assets/Roboto-Regular.ttf");

fn resolve_y(transform: Affine, font: &FontData, gid: u32) -> f32 {
    let mut scene = Scene::new();
    scene
        .draw_glyphs(font)
        .font_size(32.0)
        .transform(transform)
        .hint(true)
        .draw(
            Fill::NonZero,
            [Glyph { id: gid, x: 0.0, y: 0.0 }].into_iter(),
        );
    let mut resolver = Resolver::new();
    let mut packed = Vec::new();
    let (layout, _r, _i) = resolver.resolve(&scene.encoding(), &mut packed);
    layout.transforms(&packed)[0].translation[1]
}

fn main() {
    let font = FontData::new(Blob::new(Arc::new(ROBOTO_FONT)), 0);
    let font_ref = skrifa::FontRef::from_index(font.data.as_ref(), font.index).unwrap();
    let gid = font_ref.charmap().map('A').unwrap().to_u32();

    // Genuinely non-uniform scale: must NOT be treated as a uniform scale.
    let non_uniform = Affine::translate((100.0, 100.37)) * Affine::scale_non_uniform(2.0, 5.0);
    let y1 = resolve_y(non_uniform, &font, gid);
    println!("non-uniform scale: y={y1} (expect un-rounded 100.37-ish, hint disabled)");
    assert!(y1 != y1.round(), "FAIL: non-uniform scale was incorrectly treated as uniform");

    // Genuine skew: must NOT be treated as a uniform scale.
    let skewed = Affine::translate((100.0, 100.37)) * Affine::new([3.0, 0.5, 0.0, 3.0, 0.0, 0.0]);
    let y2 = resolve_y(skewed, &font, gid);
    println!("skewed transform: y={y2} (expect un-rounded 100.37-ish, hint disabled)");
    assert!(y2 != y2.round(), "FAIL: skewed transform was incorrectly treated as uniform");

    println!("OK: fix does not over-broaden the uniform-scale detection");
}
