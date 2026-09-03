use skrifa::raw::tables::colr::Paint;
use skrifa::raw::types::GlyphId16;
use skrifa::raw::{ResolveOffset, TableProvider};
use std::fs;

/// Find the first PaintSweepGradient (format 8, non-variable) reachable from any base glyph,
/// returning (glyph_id, absolute_start_angle_offset, absolute_end_angle_offset).
fn find_sweep_offsets(
    paint: &Paint,
    layer_list: Option<&skrifa::raw::tables::colr::LayerList>,
    depth: usize,
    base_ptr: usize,
) -> Option<(usize, usize)> {
    if depth > 16 {
        return None;
    }
    match paint {
        Paint::SweepGradient(p) => {
            let start = p.start_angle_byte_range();
            let end = p.end_angle_byte_range();
            let bytes = p.offset_data().as_bytes();
            let base = bytes.as_ptr() as usize - base_ptr;
            Some((base + start.start, base + end.start))
        }
        Paint::ColrLayers(p) => {
            let ll = layer_list?;
            let start = p.first_layer_index() as usize;
            let count = p.num_layers() as usize;
            let offsets = ll.paint_offsets();
            for i in start..start + count {
                let off_entry = offsets.get(i)?;
                let inner: Paint = off_entry.get().resolve(ll.offset_data()).ok()?;
                if let Some(off) = find_sweep_offsets(&inner, layer_list, depth + 1, base_ptr) {
                    return Some(off);
                }
            }
            None
        }
        Paint::Glyph(p) => {
            let inner = p.paint().ok()?;
            find_sweep_offsets(&inner, layer_list, depth + 1, base_ptr)
        }
        Paint::Transform(p) => {
            let inner = p.paint().ok()?;
            find_sweep_offsets(&inner, layer_list, depth + 1, base_ptr)
        }
        Paint::Translate(p) => {
            let inner = p.paint().ok()?;
            find_sweep_offsets(&inner, layer_list, depth + 1, base_ptr)
        }
        Paint::Composite(p) => {
            if let Ok(inner) = p.source_paint() {
                if let Some(off) = find_sweep_offsets(&inner, layer_list, depth + 1, base_ptr) {
                    return Some(off);
                }
            }
            if let Ok(inner) = p.backdrop_paint() {
                return find_sweep_offsets(&inner, layer_list, depth + 1, base_ptr);
            }
            None
        }
        _ => None,
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

    let mut found: Option<(u16, usize, usize)> = None;
    for rec in bgl.base_glyph_paint_records() {
        let gid: GlyphId16 = rec.glyph_id();
        if let Ok(paint) = rec.paint(bgl.offset_data()) {
            if let Some((start_off, end_off)) =
                find_sweep_offsets(&paint, layer_list.as_ref(), 0, base_ptr)
            {
                found = Some((gid.to_u32() as u16, start_off, end_off));
                break;
            }
        }
    }

    let (gid, start_off, end_off) = found.expect("found a PaintSweepGradient (format 8)");
    let start_bytes = [bytes[start_off], bytes[start_off + 1]];
    let orig_end_bytes = [bytes[end_off], bytes[end_off + 1]];
    println!(
        "glyph_id={gid} start_angle_offset={start_off} end_angle_offset={end_off} start_bytes={start_bytes:?} orig_end_bytes={orig_end_bytes:?}"
    );

    let mut patched = bytes.clone();
    patched[end_off] = start_bytes[0];
    patched[end_off + 1] = start_bytes[1];

    fs::write(&out_path, &patched).expect("write patched font");
    println!(
        "wrote patched font: glyph {gid}'s PaintSweepGradient end_angle bytes {orig_end_bytes:?} -> {start_bytes:?} (now == start_angle, degenerate zero-width sweep) to {out_path}"
    );
}
