use unicode_bidi::Level;
use unicode_bidi::ParagraphInfo;

fn main() {
    let paragraph_info = ParagraphInfo { range: usize::MAX..4294967295usize, level: Level::ltr() };
    let _ = paragraph_info.len();
}
