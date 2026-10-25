use unicode_bidi::HardcodedBidiData;
use unicode_bidi::Level;
use unicode_bidi::utf16::BidiInfo;

fn main() {
    let u8_v = Level::max_implicit_depth();
    let hardcoded_bidi_data = HardcodedBidiData;
    let vec = vec![1639u16];
    let _ = BidiInfo::new_with_data_source::<unicode_bidi::HardcodedBidiData>(&hardcoded_bidi_data, &vec, Some(Level::from(u8_v)));
}
