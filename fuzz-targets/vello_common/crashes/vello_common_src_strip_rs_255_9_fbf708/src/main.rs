use vello_common::strip::Strip;

fn main() {
    let mut strip = Strip::sentinel(0u16, 0u32);
    strip.set_alpha_idx(u32::MAX);
}
