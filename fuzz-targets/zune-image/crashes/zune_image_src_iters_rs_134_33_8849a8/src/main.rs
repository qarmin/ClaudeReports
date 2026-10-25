use zune_core::bit_depth::BitDepth;
use zune_core::colorspace::ColorSpace;
use zune_image::image::Image;

fn main() {
    let image = Image::new_frames(vec![], BitDepth::Unknown, 0usize, 0usize, ColorSpace::LumaA);
    let _ = image.pixels::<f32>();
}
