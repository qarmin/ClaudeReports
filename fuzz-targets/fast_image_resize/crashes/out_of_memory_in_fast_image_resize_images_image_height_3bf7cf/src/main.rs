use fast_image_resize::PixelType;
use fast_image_resize::images::Image;

fn main() {
    let pixel_type = PixelType::U16x4;
    let image = Image::new(1u32, 1000000000u32, pixel_type);
    let _ = image.height();
}
