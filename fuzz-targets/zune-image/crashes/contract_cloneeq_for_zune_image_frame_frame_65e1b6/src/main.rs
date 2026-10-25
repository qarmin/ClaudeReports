use zune_core::bit_depth::BitType;
use zune_image::channel::Channel;
use zune_image::frame::Frame;

fn main() {
    let frame = Frame::new_with_duration(vec![Channel::new_with_bit_type(10000usize, BitType::F32), Channel::new_with_bit_type(0usize, BitType::U16)], 0usize, 0usize);
    assert!(frame.clone() == frame, "clone is not equal to the original");
}
