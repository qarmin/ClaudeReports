use fast_image_resize::PixelType;
use fast_image_resize::images::Image;

fn main() {
    let image = Image::new(1000000000u32, 1u32, PixelType::U8x4);
    let _ = Image::from_vec_u8(0u32, 0u32, vec![], image.pixel_type());
}
