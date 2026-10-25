use unicode_bidi::LTR_LEVEL;

fn main() {
    let _ = unicode_bidi::deprecated::visual_runs(1usize..1000000000000000000usize, &[LTR_LEVEL]);
}
