use x11rb::image::BitsPerPixel;
use x11rb::image::Image;
use x11rb::image::ImageOrder;
use x11rb::image::ScanlinePad;

fn main() {
    let image = Image::allocate(0u16, 0u16, ScanlinePad::Pad8, 0u8, BitsPerPixel::B8, ImageOrder::MsbFirst);
    let image2 = image.clone().into_owned();
    let mut image3 = image2.clone().into_owned();
    image3.put_pixel(0u16, 0u16, 1u32);
}
