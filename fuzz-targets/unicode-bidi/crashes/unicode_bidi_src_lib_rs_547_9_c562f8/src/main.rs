use unicode_bidi::BidiInfo;
use unicode_bidi::HardcodedBidiData;
use unicode_bidi::Level;
use unicode_bidi::ParagraphInfo;

fn main() {
    let paragraph_info = ParagraphInfo { range: 0usize..0usize, level: Level::ltr() };
    let hardcoded_bidi_data = HardcodedBidiData;
    let string = "".to_string();
    let bidi_info = BidiInfo::new_with_data_source::<unicode_bidi::HardcodedBidiData>(&hardcoded_bidi_data, &string, None);
    let _ = bidi_info.reordered_levels(&paragraph_info, 1usize..0usize);
}
