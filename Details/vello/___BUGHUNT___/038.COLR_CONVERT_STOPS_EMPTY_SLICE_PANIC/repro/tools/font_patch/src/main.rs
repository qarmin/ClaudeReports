use skrifa::raw::tables::colr::Paint;
use skrifa::raw::types::GlyphId16;
use skrifa::raw::{FontData, FontRef, ResolveOffset, TableProvider};
use std::fs;

fn find_gradient_colorline_offset(
    paint: &Paint,
    layer_list_data: FontData,
    layer_list: Option<&skrifa::raw::tables::colr::LayerList>,
    depth: usize,
) -> Option<usize> {
    if depth > 16 {
        return None;
    }
    match paint {
        Paint::LinearGradient(p) => {
            let cl = p.color_line().ok()?;
            let bytes = cl.offset_data().as_bytes();
            Some(bytes.as_ptr() as usize)
        }
        Paint::RadialGradient(p) => {
            let cl = p.color_line().ok()?;
            let bytes = cl.offset_data().as_bytes();
            Some(bytes.as_ptr() as usize)
        }
        Paint::SweepGradient(p) => {
            let cl = p.color_line().ok()?;
            let bytes = cl.offset_data().as_bytes();
            Some(bytes.as_ptr() as usize)
        }
        Paint::ColrLayers(p) => {
            let ll = layer_list?;
            let start = p.first_layer_index() as usize;
            let count = p.num_layers() as usize;
            let offsets = ll.paint_offsets();
            for i in start..start + count {
                let off_entry = offsets.get(i)?;
                let layer_paint: Paint = off_entry.get().resolve(ll.offset_data()).ok()?;
                if let Some(off) =
                    find_gradient_colorline_offset(&layer_paint, layer_list_data, layer_list, depth + 1)
                {
                    return Some(off);
                }
            }
            None
        }
        Paint::Glyph(p) => {
            let inner = p.paint().ok()?;
            find_gradient_colorline_offset(&inner, layer_list_data, layer_list, depth + 1)
        }
        Paint::ColrGlyph(_) => None,
        Paint::Transform(p) => {
            let inner = p.paint().ok()?;
            find_gradient_colorline_offset(&inner, layer_list_data, layer_list, depth + 1)
        }
        Paint::Translate(p) => {
            let inner = p.paint().ok()?;
            find_gradient_colorline_offset(&inner, layer_list_data, layer_list, depth + 1)
        }
        Paint::Scale(p) => {
            let inner = p.paint().ok()?;
            find_gradient_colorline_offset(&inner, layer_list_data, layer_list, depth + 1)
        }
        Paint::ScaleAroundCenter(p) => {
            let inner = p.paint().ok()?;
            find_gradient_colorline_offset(&inner, layer_list_data, layer_list, depth + 1)
        }
        Paint::ScaleUniform(p) => {
            let inner = p.paint().ok()?;
            find_gradient_colorline_offset(&inner, layer_list_data, layer_list, depth + 1)
        }
        Paint::ScaleUniformAroundCenter(p) => {
            let inner = p.paint().ok()?;
            find_gradient_colorline_offset(&inner, layer_list_data, layer_list, depth + 1)
        }
        Paint::Rotate(p) => {
            let inner = p.paint().ok()?;
            find_gradient_colorline_offset(&inner, layer_list_data, layer_list, depth + 1)
        }
        Paint::RotateAroundCenter(p) => {
            let inner = p.paint().ok()?;
            find_gradient_colorline_offset(&inner, layer_list_data, layer_list, depth + 1)
        }
        Paint::Skew(p) => {
            let inner = p.paint().ok()?;
            find_gradient_colorline_offset(&inner, layer_list_data, layer_list, depth + 1)
        }
        Paint::SkewAroundCenter(p) => {
            let inner = p.paint().ok()?;
            find_gradient_colorline_offset(&inner, layer_list_data, layer_list, depth + 1)
        }
        Paint::Composite(p) => {
            if let Ok(inner) = p.source_paint() {
                if let Some(off) =
                    find_gradient_colorline_offset(&inner, layer_list_data, layer_list, depth + 1)
                {
                    return Some(off);
                }
            }
            if let Ok(inner) = p.backdrop_paint() {
                return find_gradient_colorline_offset(&inner, layer_list_data, layer_list, depth + 1);
            }
            None
        }
        _ => None,
    }
}

fn main() {
    let path = std::env::args().nth(1).expect("font path");
    let bytes = fs::read(&path).expect("read font");
    let base_ptr = bytes.as_ptr() as usize;

    let font = FontRef::new(&bytes).expect("parse font");
    let colr = font.colr().expect("colr table");
    let bgl = colr.base_glyph_list().expect("has base glyph list").expect("ok");
    let layer_list = colr.layer_list().and_then(|r| r.ok());
    let layer_list_data = layer_list
        .as_ref()
        .map(|l| l.offset_data())
        .unwrap_or_else(|| bgl.offset_data());

    let mut found: Option<(u16, usize)> = None;
    for rec in bgl.base_glyph_paint_records() {
        let gid: GlyphId16 = rec.glyph_id();
        let paint = rec.paint(bgl.offset_data()).expect("paint");
        if let Some(off) =
            find_gradient_colorline_offset(&paint, layer_list_data, layer_list.as_ref(), 0)
        {
            found = Some((gid.to_u32() as u16, off - base_ptr));
            break;
        }
    }

    let (gid, colorline_abs_offset) = found.expect("found a gradient colorline");
    println!("glyph_id={gid} colorline_abs_offset={colorline_abs_offset}");

    // ColorLine layout: extend:u8 (1 byte), numStops:u16 (2 bytes), stops...
    let num_stops_offset = colorline_abs_offset + 1;
    let orig_num_stops = u16::from_be_bytes([bytes[num_stops_offset], bytes[num_stops_offset + 1]]);
    println!("original numStops={orig_num_stops}");

    let mut patched = bytes.clone();
    patched[num_stops_offset] = 0;
    patched[num_stops_offset + 1] = 0;

    let out_path = std::env::args().nth(2).expect("output path");
    fs::write(&out_path, &patched).expect("write patched font");
    println!("wrote patched font with numStops=0 for glyph {gid} to {out_path}");
}
