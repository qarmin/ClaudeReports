use zune_core::bit_depth::BitType;
use zune_image::channel::Channel;

fn main() {
    let _ = zune_image::utils::swizzle_channels::<u8>(&[Channel::new_with_bit_type(0usize, BitType::U8)], &mut vec![0u8]);
}
