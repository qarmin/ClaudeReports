use epaint::ColorImage;

fn main() {
    let _ = ColorImage::from_rgba_premultiplied([usize::MAX, 16usize], &include_bytes!("image_0.gif").to_vec());
}
