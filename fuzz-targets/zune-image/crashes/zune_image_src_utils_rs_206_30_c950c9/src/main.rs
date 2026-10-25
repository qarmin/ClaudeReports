use zune_core::bit_depth::BitType;
use zune_image::channel::Channel;

fn main() {
    let channel = Channel::new_with_bit_type(0usize, BitType::U8);
    let _ = zune_image::utils::swizzle_channels::<u8>(&[channel.clone(), channel.clone(), Channel::new_with_bit_type(0usize, BitType::U8), channel.clone(), channel.clone()], &mut vec![0u8, 80u8, 78u8, 71u8, 0u8]);
}
