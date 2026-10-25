use unicode_bidi::BidiInfo;
use unicode_bidi::Level;
use unicode_bidi::ParagraphInfo;

fn main() {
    let paragraph_info = ParagraphInfo { range: 0usize..0usize, level: Level::new(0u8).unwrap() };
    let string = "\u{896db}".to_string();
    let bidi_info = BidiInfo::new(&string, None);
    let _ = bidi_info.reorder_line(&paragraph_info, 1usize..1usize);
}
