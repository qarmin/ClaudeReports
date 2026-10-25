use vello_common::strip::Strip;

fn main() {
    let _ = vello_common::util::strip_bbox(&[Strip::new(0u16, 0u16, 0u32, false), Strip::new(0u16, 0u16, 100u32, false)]);
}
