use unicode_bidi::Level;

fn main() {
    let _ = unicode_bidi::deprecated::visual_runs(usize::MAX..4294967296usize, &[Level::ltr()]);
}
