use fast_image_resize::PixelType;
use fast_image_resize::images::Image;

fn main() {
    let _ = Image::from_slice_u8(u32::MAX, 1000000000u32, &mut vec![], PixelType::F32x2);
}
