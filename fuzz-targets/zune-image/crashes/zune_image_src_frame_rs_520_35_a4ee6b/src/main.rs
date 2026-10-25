use zune_image::frame::Endianness;
use zune_image::frame::Frame;

fn main() {
    let frame = Frame::new_with_duration(vec![], 0usize, 0usize);
    let _ = frame.u16_to_u8_endian(Endianness::Native);
}
