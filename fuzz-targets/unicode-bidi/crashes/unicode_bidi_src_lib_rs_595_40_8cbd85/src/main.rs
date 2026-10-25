use unicode_bidi::BidiInfo;
use unicode_bidi::HardcodedBidiData;
use unicode_bidi::Level;
use unicode_bidi::ParagraphInfo;

fn main() {
    let hardcoded_bidi_data = HardcodedBidiData;
    let string = "".to_string();
    let bidi_info = BidiInfo::new_with_data_source::<unicode_bidi::HardcodedBidiData>(&hardcoded_bidi_data, &string, None);
    let _ = bidi_info.reorder_line(&ParagraphInfo { range: 1usize..0usize, level: Level::new_explicit(0u8).unwrap() }, 0usize..1usize);
}
