use zune_core::bit_depth::BitType;
use zune_image::channel::Channel;

fn main() {
    let _ = Channel::new_with_bit_type(usize::MAX, BitType::U8);
}
