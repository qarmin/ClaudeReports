use unicode_bidi::BidiInfo;
use unicode_bidi::Level;
use unicode_bidi::ParagraphInfo;

fn main() {
    let string = "".to_string();
    let bidi_info = BidiInfo::new(&string, None);
    let _ = bidi_info.visual_runs(&ParagraphInfo { range: 0usize..0usize, level: Level::from(0u8) }, 0usize..1usize);
}
