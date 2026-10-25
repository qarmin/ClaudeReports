use fast_image_resize::PixelType;
use fast_image_resize::images::Image;

fn main() {
    let image = Image::new(1u32, 1000000000u32, PixelType::U16x2);
    let _ = image.copy();
}
