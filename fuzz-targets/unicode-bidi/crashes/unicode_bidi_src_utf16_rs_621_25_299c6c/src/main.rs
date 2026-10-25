use unicode_bidi::HardcodedBidiData;
use unicode_bidi::Level;
use unicode_bidi::ParagraphInfo;
use unicode_bidi::utf16::BidiInfo;
use unicode_bidi::utf16::Paragraph;

fn main() {
    let hardcoded_bidi_data = HardcodedBidiData;
    let vec = vec![];
    let bidi_info = BidiInfo::new_with_data_source::<unicode_bidi::HardcodedBidiData>(&hardcoded_bidi_data, &vec, None);
    let paragraph_info = ParagraphInfo { range: 0usize..0usize, level: Level::ltr() };
    let paragraph = Paragraph::new(&bidi_info, &paragraph_info);
    let _ = paragraph.level_at(0usize);
}
