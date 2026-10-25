use unicode_bidi::HardcodedBidiData;
use unicode_bidi::ParagraphBidiInfo;

fn main() {
    let hardcoded_bidi_data = HardcodedBidiData;
    let string = "a".to_string();
    let paragraph_bidi_info = ParagraphBidiInfo::new_with_data_source::<unicode_bidi::HardcodedBidiData>(&hardcoded_bidi_data, &string, None);
    let _ = paragraph_bidi_info.reordered_levels(1usize..0usize);
}
