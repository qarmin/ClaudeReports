use unicode_bidi::LTR_LEVEL;
use unicode_bidi::Level;

fn main() {
    let _ = unicode_bidi::deprecated::visual_runs(2usize..0usize, &[Level::ltr(), LTR_LEVEL]);
}
