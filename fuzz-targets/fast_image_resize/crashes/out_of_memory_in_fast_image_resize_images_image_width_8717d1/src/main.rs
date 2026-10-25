use fast_image_resize::PixelType;
use fast_image_resize::images::Image;

fn main() {
    let image = Image::new(1000000000u32, 1u32, PixelType::I32);
    let _ = image.width();
}
