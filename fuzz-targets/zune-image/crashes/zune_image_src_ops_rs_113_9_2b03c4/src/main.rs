use zune_core::bit_depth::BitDepth;
use zune_core::colorspace::ColorSpace;
use zune_image::image::Image;

fn main() {
    let _ = <zune_image::image::Image as ::core::ops::Sub<zune_image::image::Image>>::sub(Image::new_frames(vec![], BitDepth::Sixteen, 0usize, 0usize, ColorSpace::HSL), Image::new_frames(vec![], BitDepth::Sixteen, 0usize, 1usize, ColorSpace::HSL));
}
