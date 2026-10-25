use unicode_bidi::utf16::ParagraphBidiInfo;

fn main() {
    let vec = vec![];
    let paragraph_bidi_info = ParagraphBidiInfo::new(&vec, None);
    let _ = paragraph_bidi_info.reordered_levels_per_char(0usize..1usize);
}
