use vello_common::strip::Strip;

fn main() {
    let _ = vello_common::util::strip_bbox(&[Strip::sentinel(1u16, 0u32), Strip::new(0u16, 0u16, 0u32, false)]);
}
