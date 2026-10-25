use unicode_bidi::HardcodedBidiData;
use unicode_bidi::LTR_LEVEL;
use unicode_bidi::ParagraphInfo;
use unicode_bidi::utf16::BidiInfo;

fn main() {
    let hardcoded_bidi_data = HardcodedBidiData;
    let vec = vec![];
    let bidi_info = BidiInfo::new_with_data_source::<unicode_bidi::HardcodedBidiData>(&hardcoded_bidi_data, &vec, None);
    let _ = bidi_info.visual_runs(&ParagraphInfo { range: 0usize..0usize, level: LTR_LEVEL }, 0usize..0usize);
}
