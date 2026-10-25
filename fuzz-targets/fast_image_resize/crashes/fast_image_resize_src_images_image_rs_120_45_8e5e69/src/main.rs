use fast_image_resize::PixelType;
use fast_image_resize::images::Image;

fn main() {
    let _ = Image::new(1000000000u32, u32::MAX, PixelType::U8x4);
}
