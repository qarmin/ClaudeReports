use fast_image_resize::PixelType;
use fast_image_resize::images::Image;

fn main() {
    let image = Image::new(1u32, 1997974991u32, PixelType::U8x2);
    let _ = Image::from_slice_u8(1u32, 0u32, &mut vec![], image.pixel_type());
}
