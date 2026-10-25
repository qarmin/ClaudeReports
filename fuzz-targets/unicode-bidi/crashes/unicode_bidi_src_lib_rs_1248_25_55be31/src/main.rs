use unicode_bidi::BidiInfo;
use unicode_bidi::Level;
use unicode_bidi::Paragraph;
use unicode_bidi::ParagraphInfo;

fn main() {
    let paragraph_info = ParagraphInfo { range: 0usize..0usize, level: Level::from(1u8) };
    let string = "".to_string();
    let bidi_info = BidiInfo::new(&string, None);
    let paragraph = Paragraph::new(&bidi_info, &paragraph_info);
    let _ = paragraph.level_at(0usize);
}
