use unicode_bidi::Level;
use unicode_bidi::ParagraphInfo;
use unicode_bidi::utf16::ParagraphBidiInfo;

fn main() {
    let paragraph_info = ParagraphInfo { range: 65535usize..65535usize, level: Level::new_explicit(0u8).unwrap() };
    let usize_v = paragraph_info.len();
    let vec = vec![0u16];
    let paragraph_bidi_info = ParagraphBidiInfo::new(&vec, None);
    let _ = paragraph_bidi_info.reordered_levels_per_char(1usize..usize_v);
}
