use zune_core::bit_depth::BitType;
use zune_core::colorspace::ColorSpace;
use zune_image::channel::Channel;
use zune_image::frame::Frame;

fn main() {
    let mut frame = Frame::from_u8(&include_bytes!("data_0.bin").to_vec(), ColorSpace::RGB, 0usize, 0usize);
    frame.set_channels(vec![Channel::new_with_bit_type(4294967296usize, BitType::F32)]);
}
