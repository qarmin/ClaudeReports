use unicode_bidi::ParagraphBidiInfo;

fn main() {
    let string = "\u{feff}".to_string();
    let paragraph_bidi_info = ParagraphBidiInfo::new(&string, None);
    let _ = paragraph_bidi_info.reorder_line(0usize..1usize);
}
