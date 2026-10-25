use unicode_bidi::BidiInfo;
use unicode_bidi::Level;

fn main() {
    let u8_v = Level::max_implicit_depth();
    let string = "ﰍ".to_string();
    let _ = BidiInfo::new(&string, Some(Level::from(u8_v)));
}
