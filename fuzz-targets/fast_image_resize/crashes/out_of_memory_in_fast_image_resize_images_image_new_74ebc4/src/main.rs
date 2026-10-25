use fast_image_resize::PixelType;
use fast_image_resize::images::Image;

fn main() {
    let _ = Image::new(1u32, 1000000000u32, PixelType::I32);
}
