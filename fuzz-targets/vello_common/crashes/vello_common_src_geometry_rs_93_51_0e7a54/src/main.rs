use vello_common::geometry::SizeU16;

fn main() {
    let _ = <vello_common::geometry::SizeU16 as ::core::ops::Add<vello_common::geometry::SizeU16>>::add(SizeU16([u16::MAX, 0u16]), SizeU16::new(1u16));
}
