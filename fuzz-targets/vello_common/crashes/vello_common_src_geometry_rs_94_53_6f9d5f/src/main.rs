use vello_common::geometry::SizeU16;

fn main() {
    let _ = <vello_common::geometry::SizeU16 as ::core::ops::Add<u16>>::add(SizeU16::from_wh(0u16, u16::MAX), 1u16);
}
