use fast_image_resize::MulDiv;
use fast_image_resize::PixelType;
use fast_image_resize::images::Image;

fn main() {
    let mul_div = MulDiv::new();
    let image = Image::new(7u32, 100000000u32, PixelType::U16x4);
    let _ = mul_div.is_supported(image.pixel_type());
}
