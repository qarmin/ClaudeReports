use swash::FontRef;

fn main() {
    let vec = vec![255u8, 216u8, 255u8, 224u8, 0u8, 16u8, 74u8, 70u8, 73u8, 70u8, 0u8, 1u8, 2u8, 0u8, 0u8, 1u8, 0u8, 1u8, 0u8, 0u8];
    let font_ref = FontRef::from_offset(&vec, 16u32).unwrap();
    let vec2 = vec![];
    let glyph_metrics = font_ref.glyph_metrics(&vec2);
    let _ = glyph_metrics.advance_width(0u16);
}
