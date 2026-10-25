use fast_image_resize::PixelType;
use fast_image_resize::images::Image;

fn main() {
    let pixel_type = PixelType::F32x4;
    let _ = Image::from_vec_u8(1000000000u32, 1572920769u32, vec![], pixel_type);
}
