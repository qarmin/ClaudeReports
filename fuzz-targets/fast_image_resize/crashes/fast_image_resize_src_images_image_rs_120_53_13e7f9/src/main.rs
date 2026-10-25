use fast_image_resize::PixelType;
use fast_image_resize::images::Image;

fn main() {
    let _ = Image::new(u32::MAX, u32::MAX, PixelType::U16x2);
}
