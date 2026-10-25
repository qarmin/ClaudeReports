use image::math::Rect;

fn main() {
    let mut image_buffer = <image::ImageBuffer<image::Luma<u16>, Vec<u16>> as ::core::default::Default>::default();
    let _ = <image::ImageBuffer<image::Luma<u16>, Vec<u16>> as image::GenericImage>::sub_image(&mut image_buffer, Rect { x: 0u32, y: 0u32, width: 0u32, height: 1u32 });
}
