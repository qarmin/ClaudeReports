use unicode_bidi::BidiInfo;
use unicode_bidi::ParagraphInfo;
use unicode_bidi::RTL_LEVEL;

fn main() {
    let string = "Z".to_string();
    let bidi_info = BidiInfo::new(&string, None);
    let _ = bidi_info.reordered_levels_per_char(&ParagraphInfo { range: 0usize..0usize, level: RTL_LEVEL }, 1usize..0usize);
}
