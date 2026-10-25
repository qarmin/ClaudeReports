use unicode_bidi::HardcodedBidiData;
use unicode_bidi::Level;
use unicode_bidi::ParagraphInfo;
use unicode_bidi::utf16::BidiInfo;

fn main() {
    let hardcoded_bidi_data = HardcodedBidiData;
    let vec = vec![0u16];
    let bidi_info = BidiInfo::new_with_data_source::<unicode_bidi::HardcodedBidiData>(&hardcoded_bidi_data, &vec, None);
    let _ = bidi_info.reordered_levels(&ParagraphInfo { range: 0usize..0usize, level: Level::new(0u8).unwrap() }, 1usize..0usize);
}
