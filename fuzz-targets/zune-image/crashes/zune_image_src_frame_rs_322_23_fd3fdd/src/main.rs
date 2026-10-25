use zune_core::bit_depth::BitType;
use zune_image::channel::Channel;
use zune_image::frame::Frame;

fn main() {
    let bit_type = BitType::F32;
    let mut frame = Frame::new_with_duration(vec![], 0usize, 0usize);
    frame.insert(1usize, Channel::new_with_bit_type(0usize, bit_type));
}
