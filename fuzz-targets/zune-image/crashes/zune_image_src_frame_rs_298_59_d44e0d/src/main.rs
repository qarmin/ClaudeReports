use zune_core::colorspace::ColorSpace;
use zune_image::frame::Frame;

fn main() {
    let mut frame = Frame::from_u16(&[1u16, 0u16, 0u16], ColorSpace::HSL, 0usize, 0usize);
    let _ = frame.channels_mut(ColorSpace::LumaA, true);
}
