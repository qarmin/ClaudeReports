use skrifa::raw::tables::colr::Paint;
use skrifa::raw::types::GlyphId16;
use skrifa::raw::{ResolveOffset, TableProvider};
use std::fs;

struct RadialInfo {
    glyph_id: u16,
    r0: f32,
    r1: f32,
    x0: f32,
    y0: f32,
    x1: f32,
    y1: f32,
    r1_off: usize,
    r0_off: usize,
}

/// Find every PaintRadialGradient (format 6, non-variable) reachable from `paint`, appending each
/// one found to `out` (tagged with `glyph_id`).
fn find_radial(
    paint: &Paint,
    glyph_id: u16,
    layer_list: Option<&skrifa::raw::tables::colr::LayerList>,
    depth: usize,
    base_ptr: usize,
    out: &mut Vec<RadialInfo>,
) {
    if depth > 16 {
        return;
    }
    match paint {
        Paint::RadialGradient(p) => {
            let r1_range = p.radius1_byte_range();
            let r0_range = p.radius0_byte_range();
            let bytes = p.offset_data().as_bytes();
            let base = bytes.as_ptr() as usize - base_ptr;
            out.push(RadialInfo {
                glyph_id,
                r0: p.radius0().to_u16() as f32,
                r1: p.radius1().to_u16() as f32,
                x0: p.x0().to_i16() as f32,
                y0: p.y0().to_i16() as f32,
                x1: p.x1().to_i16() as f32,
                y1: p.y1().to_i16() as f32,
                r1_off: base + r1_range.start,
                r0_off: base + r0_range.start,
            });
        }
        Paint::ColrLayers(p) => {
            let Some(ll) = layer_list else { return };
            let start = p.first_layer_index() as usize;
            let count = p.num_layers() as usize;
            let offsets = ll.paint_offsets();
            for i in start..start + count {
                let Some(off_entry) = offsets.get(i) else {
                    continue;
                };
                let Ok(inner) = off_entry.get().resolve::<Paint>(ll.offset_data()) else {
                    continue;
                };
                find_radial(&inner, glyph_id, layer_list, depth + 1, base_ptr, out);
            }
        }
        Paint::Glyph(p) => {
            if let Ok(inner) = p.paint() {
                find_radial(&inner, glyph_id, layer_list, depth + 1, base_ptr, out);
            }
        }
        Paint::Transform(p) => {
            if let Ok(inner) = p.paint() {
                find_radial(&inner, glyph_id, layer_list, depth + 1, base_ptr, out);
            }
        }
        Paint::Translate(p) => {
            if let Ok(inner) = p.paint() {
                find_radial(&inner, glyph_id, layer_list, depth + 1, base_ptr, out);
            }
        }
        Paint::Composite(p) => {
            if let Ok(inner) = p.source_paint() {
                find_radial(&inner, glyph_id, layer_list, depth + 1, base_ptr, out);
            }
            if let Ok(inner) = p.backdrop_paint() {
                find_radial(&inner, glyph_id, layer_list, depth + 1, base_ptr, out);
            }
        }
        _ => {}
    }
}

fn main() {
    let path = std::env::args().nth(1).expect("font path");
    let out_path = std::env::args().nth(2).expect("output path");

    let bytes = fs::read(&path).expect("read font");
    let base_ptr = bytes.as_ptr() as usize;

    let font = skrifa::raw::FontRef::new(&bytes).expect("parse font");
    let colr = font.colr().expect("colr table");
    let bgl = colr
        .base_glyph_list()
        .expect("has base glyph list")
        .expect("ok");
    let layer_list = colr.layer_list().and_then(|r| r.ok());

    let mut all = Vec::new();
    for rec in bgl.base_glyph_paint_records() {
        let gid: GlyphId16 = rec.glyph_id();
        if let Ok(paint) = rec.paint(bgl.offset_data()) {
            find_radial(
                &paint,
                gid.to_u32() as u16,
                layer_list.as_ref(),
                0,
                base_ptr,
                &mut all,
            );
        }
    }

    for info in &all {
        println!(
            "glyph {} PaintRadialGradient: c0=({},{}) r0={} c1=({},{}) r1={} r0_off={} r1_off={}",
            info.glyph_id, info.x0, info.y0, info.r0, info.x1, info.y1, info.r1, info.r0_off, info.r1_off
        );
    }

    // Prefer a gradient where r0 > 0 and r1 > 0 (so zeroing r1 alone produces a genuine
    // "start circle -> end point" cone, not two coincident degenerate circles).
    let chosen = all
        .iter()
        .find(|i| i.r0 > 0.5 && i.r1 > 0.5)
        .or_else(|| all.iter().find(|i| i.r1 > 0.5))
        .expect("found a PaintRadialGradient (format 6) with r1 > 0.5");

    println!(
        "chosen: glyph_id={} r0={} r1={} r1_offset={}",
        chosen.glyph_id, chosen.r0, chosen.r1, chosen.r1_off
    );

    let mut patched = bytes.clone();
    let orig_bytes = [patched[chosen.r1_off], patched[chosen.r1_off + 1]];
    // UFWORD (u16, big-endian): set radius1 to 0.
    patched[chosen.r1_off] = 0;
    patched[chosen.r1_off + 1] = 0;
    // If r0 was originally 0 too, bump it so the two circles are not both degenerate points.
    if chosen.r0 < 0.5 {
        patched[chosen.r0_off] = 0;
        patched[chosen.r0_off + 1] = 128;
    }

    fs::write(&out_path, &patched).expect("write patched font");
    println!(
        "wrote patched font: glyph {}'s PaintRadialGradient.radius1 bytes {orig_bytes:?} -> [0, 0] \
         (end radius now 0.0, a real spec-legal 'focal point' cone gradient), r0={} to {out_path}",
        chosen.glyph_id,
        if chosen.r0 < 0.5 { 128.0 } else { chosen.r0 },
    );
}
