use skrifa::raw::tables::colr::Paint;
use skrifa::raw::types::GlyphId16;
use skrifa::raw::{ResolveOffset, TableProvider};
use std::fs;

fn find_solid_offset(
    paint: &Paint,
    layer_list: Option<&skrifa::raw::tables::colr::LayerList>,
    depth: usize,
    base_ptr: usize,
) -> Option<usize> {
    if depth > 16 {
        return None;
    }
    match paint {
        Paint::Solid(p) => {
            let bytes = p.offset_data().as_bytes();
            Some(bytes.as_ptr() as usize - base_ptr)
        }
        Paint::ColrLayers(p) => {
            let ll = layer_list?;
            let start = p.first_layer_index() as usize;
            let count = p.num_layers() as usize;
            let offsets = ll.paint_offsets();
            for i in start..start + count {
                let off_entry = offsets.get(i)?;
                let inner: Paint = off_entry.get().resolve(ll.offset_data()).ok()?;
                if let Some(off) = find_solid_offset(&inner, layer_list, depth + 1, base_ptr) {
                    return Some(off);
                }
            }
            None
        }
        Paint::Glyph(p) => {
            let inner = p.paint().ok()?;
            find_solid_offset(&inner, layer_list, depth + 1, base_ptr)
        }
        Paint::Transform(p) => {
            let inner = p.paint().ok()?;
            find_solid_offset(&inner, layer_list, depth + 1, base_ptr)
        }
        Paint::Translate(p) => {
            let inner = p.paint().ok()?;
            find_solid_offset(&inner, layer_list, depth + 1, base_ptr)
        }
        Paint::Composite(p) => {
            if let Ok(inner) = p.source_paint() {
                if let Some(off) = find_solid_offset(&inner, layer_list, depth + 1, base_ptr) {
                    return Some(off);
                }
            }
            if let Ok(inner) = p.backdrop_paint() {
                return find_solid_offset(&inner, layer_list, depth + 1, base_ptr);
            }
            None
        }
        _ => None,
    }
}

fn main() {
    let path = std::env::args().nth(1).expect("font path");
    let out_path = std::env::args().nth(2).expect("output path");
    let new_index: u16 = std::env::args()
        .nth(3)
        .expect("new palette index")
        .parse()
        .expect("u16");

    let bytes = fs::read(&path).expect("read font");
    let base_ptr = bytes.as_ptr() as usize;

    let font = skrifa::raw::FontRef::new(&bytes).expect("parse font");
    let cpal = font.cpal().expect("cpal table");
    let total_records = cpal
        .color_records_array()
        .map(|r| r.map(|a| a.len()).unwrap_or(0))
        .unwrap_or(0);
    println!(
        "CPAL num_palette_entries={} num_palettes={} color_records_array_len={total_records}",
        cpal.num_palette_entries(),
        cpal.num_palettes()
    );

    let colr = font.colr().expect("colr table");
    let bgl = colr
        .base_glyph_list()
        .expect("has base glyph list")
        .expect("ok");
    let layer_list = colr.layer_list().and_then(|r| r.ok());

    let mut found: Option<(u16, usize, u16)> = None;
    for rec in bgl.base_glyph_paint_records() {
        let gid: GlyphId16 = rec.glyph_id();
        if let Ok(paint) = rec.paint(bgl.offset_data()) {
            if let Some(off) = find_solid_offset(&paint, layer_list.as_ref(), 0, base_ptr) {
                let cur_index = u16::from_be_bytes([bytes[off + 1], bytes[off + 2]]);
                if cur_index != 0xFFFF {
                    found = Some((gid.to_u32() as u16, off, cur_index));
                    break;
                }
            }
        }
    }

    let (gid, solid_offset, orig_index) = found.expect("found a non-foreground PaintSolid");
    println!("glyph_id={gid} paintsolid_abs_offset={solid_offset} orig_palette_index={orig_index}");
    assert!(
        (new_index as usize) >= total_records,
        "new_index must be out of bounds of color_records_array (len={total_records}) to reproduce the bug"
    );

    let mut patched = bytes.clone();
    let index_offset = solid_offset + 1;
    let new_bytes = new_index.to_be_bytes();
    patched[index_offset] = new_bytes[0];
    patched[index_offset + 1] = new_bytes[1];

    fs::write(&out_path, &patched).expect("write patched font");
    println!(
        "wrote patched font: glyph {gid}'s PaintSolid.palette_index changed {orig_index} -> {new_index} (CPAL only has {total_records} color records) to {out_path}"
    );
}
