use fast_image_resize::PixelType;
use fast_image_resize::images::Image;
use fast_image_resize::images::ImageRef;

fn main() {
    let vec = vec![];
    let image = Image::new(1u32, 1000000000u32, PixelType::U8x3);
    let _ = ImageRef::new(0u32, 0u32, &vec, image.pixel_type());
}
