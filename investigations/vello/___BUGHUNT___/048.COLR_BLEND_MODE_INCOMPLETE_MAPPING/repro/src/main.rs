// Reproduction for candidate 048: COLR_BLEND_MODE_INCOMPLETE_MAPPING.
//
// `vello::scene::DrawColorGlyphs::push_layer` (the `ColorPainter` impl used by classic vello's
// `Scene::draw_glyphs(..).draw(..)` to render COLRv1 glyphs) maps `skrifa::color::CompositeMode`
// to a `peniko::Compose` value only, discarding the CSS "Mix" blend component - all 15 non-
// Porter-Duff composite modes (Multiply, Screen, Overlay, ...) fall through the wildcard arm to
// plain `Compose::SrcOver`, i.e. `Mix::Normal`, instead of `BlendMode::new(Mix::<mode>,
// Compose::SrcOver)`.
//
// This drives the REAL end-to-end path: a real COLRv1 test font (Google's
// `test_glyphs-glyf_colr_1.ttf`, already used by this project's own `vello_sparse_tests` COLR
// snapshot test) is handed to vello's real public `Scene::draw_glyphs(..).draw(..)` entry point,
// which internally runs the real `skrifa` paint-graph traversal that calls
// `ColorPainter::push_layer(CompositeMode::Multiply)` for glyph `composite_MULTIPLY` (glyph id
// 143). This is a single-participant case per CONFIRMATION.md section 11 - a font file is a
// single external artifact handed to a real public entry point, no second participant is faked.
//
// Rather than doing a full GPU render (also demonstrated to work in this sandbox - see
// RESULT.md's "What was tried" table for the pixel-level confirmation), this decodes the
// `Encoding` that `Scene::draw_glyphs` produced (both `Encoding` and `DrawBeginClip`/`DrawTag`
// are public API of the separately-published `vello_encoding` crate) to check exactly what
// `blend_mode` word got written for the `PaintComposite(mode=Multiply)` layer - this is the
// precise, deterministic root-cause-level assertion of the bug, not just its downstream visual
// symptom.

use std::iter;
use std::sync::Arc;

use vello::Glyph;
use vello::Scene;
use vello::kurbo::Affine;
use vello::peniko::{Blob, Fill, FontData};
use vello_encoding::{DrawBeginClip, DrawTag, Encoding};

const FONT_BYTES: &[u8] =
    include_bytes!("../assets/test_glyphs-glyf_colr_1.ttf");

/// Glyph id of the `composite_MULTIPLY` base glyph in `test_glyphs-glyf_colr_1.ttf`, found via
/// `fontTools` introspection of the font's `COLR` table (see RESULT.md). Its paint graph is:
///
/// ```text
/// PaintColrLayers[
///     PaintGlyph(cross_glyph, PaintSolid(black)),
///     PaintComposite(
///         mode = Multiply,
///         source   = PaintScaleUniformAroundCenter(PaintGlyph(upem_box_glyph, PaintSolid(light-blue))),
///         backdrop = PaintScaleUniformAroundCenter(PaintGlyph(upem_box_glyph, PaintSolid(yellow))),
///     ),
/// ]
/// ```
const MULTIPLY_GLYPH_ID: u32 = 143;

/// `blend_mode` packs `(mix << 8) | compose` - see `vello_encoding::DrawBeginClip::new`.
const COMPOSE_SRC_OVER: u32 = 3; // peniko::Compose::SrcOver as u32
const MIX_NORMAL: u32 = 0; // peniko::Mix::Normal as u32
const MIX_MULTIPLY: u32 = 1; // peniko::Mix::Multiply as u32
/// `DrawBeginClip::CLIP_BLEND_MODE` sentinel used by plain (non-blend) clip layers -
/// `push_clip_layer`/`push_clip_box` - so it never collides with a real `Mix` value.
const CLIP_SENTINEL_MIX: u32 = 128;

/// Walks the real encoded draw-tag/draw-data streams and returns the raw `blend_mode` word of
/// every `BEGIN_CLIP` entry, in encoding order.
///
/// This glyph's paint graph only ever produces `DrawTag::COLOR` (solid fills, 1 draw_data word),
/// `DrawTag::BEGIN_CLIP` (2 draw_data words, decoded as `DrawBeginClip`) and `DrawTag::END_CLIP`
/// (0 draw_data words) - see `vello_encoding::Encoding::encode_color`/`encode_begin_clip`/
/// `encode_end_clip`. Any other tag would mean this glyph's paint graph doesn't match what was
/// found via `fontTools` and the repro needs updating, so it panics loudly instead of silently
/// misreading the stream.
fn decode_begin_clip_blend_modes(encoding: &Encoding) -> Vec<u32> {
    let mut out = Vec::new();
    let mut data = encoding.draw_data.iter().copied();
    for tag in &encoding.draw_tags {
        match *tag {
            DrawTag::COLOR => {
                data.next().expect("DrawColor: missing rgba word");
            }
            DrawTag::BEGIN_CLIP => {
                let w0 = data.next().expect("DrawBeginClip: missing word 0");
                let w1 = data.next().expect("DrawBeginClip: missing word 1");
                let mut bytes = [0_u8; 8];
                bytes[0..4].copy_from_slice(&w0.to_ne_bytes());
                bytes[4..8].copy_from_slice(&w1.to_ne_bytes());
                let clip: DrawBeginClip = bytemuck::pod_read_unaligned(&bytes);
                out.push(clip.blend_mode);
            }
            DrawTag::END_CLIP | DrawTag::NOP => {}
            other => panic!(
                "repro assumption violated: unexpected DrawTag(0x{:x}) in this glyph's \
                 encoding - repro needs updating",
                other.0
            ),
        }
    }
    out
}

fn main() {
    let font = FontData::new(Blob::new(Arc::new(FONT_BYTES.to_vec())), 0);

    let mut scene = Scene::new();
    scene.draw_glyphs(&font).font_size(40.0).transform(Affine::IDENTITY).draw(
        Fill::NonZero,
        iter::once(Glyph {
            id: MULTIPLY_GLYPH_ID,
            x: 0.0,
            y: 0.0,
        }),
    );

    let blend_modes = decode_begin_clip_blend_modes(scene.encoding());
    println!("Encoded BEGIN_CLIP blend_mode words: {:#06x?}", blend_modes);

    // `skrifa`'s own paint-graph traversal (`skrifa-0.44.0/src/color/traversal.rs`,
    // `ResolvedPaint::Composite` arm) always emits *two* consecutive `push_layer` calls per
    // `PaintComposite` node: an outer `push_layer(CompositeMode::SrcOver)` wrapper (always
    // correctly mapped, since `SrcOver` is one of the explicitly-handled Porter-Duff arms), then
    // the real `push_layer(*mode)` call for the paint's actual mode (`Multiply` here, the one
    // whose mapping this bug affects) around the "source" paint. Both therefore encode
    // `Compose::SrcOver` in the low byte; the *second* (last) one is the one under test.
    let non_clip_src_over_layers: Vec<u32> = blend_modes
        .iter()
        .copied()
        .filter(|bm| bm & 0xff == COMPOSE_SRC_OVER && (bm >> 8) != CLIP_SENTINEL_MIX)
        .collect();
    assert_eq!(
        non_clip_src_over_layers.len(),
        2,
        "expected exactly 2 non-clip SrcOver-composited layers (the PaintComposite node's outer \
         CompositeMode::SrcOver wrapper, then its own Multiply-mode layer) - got {:#06x?}, repro \
         assumption violated",
        non_clip_src_over_layers
    );
    let composite_layer = non_clip_src_over_layers[1];

    let mix = composite_layer >> 8;
    println!(
        "PaintComposite(mode=Multiply) was encoded as Compose::SrcOver + Mix={mix} \
         (expected Mix::Multiply={MIX_MULTIPLY}; Mix::Normal={MIX_NORMAL} is what the bug produces)"
    );

    if mix == MIX_MULTIPLY {
        println!("did not reproduce - Mix::Multiply was correctly encoded");
    } else if mix == MIX_NORMAL {
        eprintln!(
            "REPRODUCED - skrifa::color::CompositeMode::Multiply was mapped to plain \
             Compose::SrcOver (Mix::Normal) instead of BlendMode::new(Mix::Multiply, \
             Compose::SrcOver)"
        );
        std::process::exit(1);
    } else {
        panic!("unexpected Mix value {mix} - repro assumption violated");
    }
}
