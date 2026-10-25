use unicode_bidi::BidiInfo;
use unicode_bidi::HardcodedBidiData;
use unicode_bidi::Level;
use unicode_bidi::Paragraph;
use unicode_bidi::ParagraphInfo;

fn main() {
    let hardcoded_bidi_data = HardcodedBidiData;
    let string = "".to_string();
    let bidi_info = BidiInfo::new_with_data_source::<unicode_bidi::HardcodedBidiData>(&hardcoded_bidi_data, &string, None);
    let paragraph_info = ParagraphInfo { range: 0usize..1usize, level: Level::rtl() };
    let paragraph = Paragraph::new(&bidi_info, &paragraph_info);
    let _ = paragraph.direction();
}
