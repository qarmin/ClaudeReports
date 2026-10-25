use zune_image::frame::Endianness;
use zune_image::frame::Frame;

fn main() {
    let frame = Frame::new(vec![]);
    let _ = frame.u16_to_u8_endian_into(Endianness::Little, &mut vec![]);
}
