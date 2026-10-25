use zune_core::colorspace::ColorSpace;
use zune_image::image::Image;

fn main() {
    let _ = Image::fill::<f32>(0.0f32, ColorSpace::BGR, 11usize, 16777217usize);
}
