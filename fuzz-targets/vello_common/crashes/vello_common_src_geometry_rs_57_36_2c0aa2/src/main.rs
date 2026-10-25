use vello_common::geometry::SizeU16;

fn main() {
    let size_u16 = SizeU16::from((0u16, 0u16));
    let _ = size_u16.clamp(1u16, 0u16);
}
