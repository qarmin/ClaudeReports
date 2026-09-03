use image::{DynamicImage, GenericImageView, RgbaImage};

fn main() {
    let w: u32 = 1;
    let h: u32 = 1_000_000;
    println!("building source image {}x{} ({} pixels, {} bytes)", w, h, w as u64 * h as u64, w as u64 * h as u64 * 4);
    let buf = RgbaImage::new(w, h);
    let mut img = DynamicImage::ImageRgba8(buf);
    println!("source built, calling resize_to_fill(100, 100, Triangle)");
    img.resize_to_fill(100, 100, image::imageops::FilterType::Triangle);
    println!("did not OOM - result dims: {:?}", img.dimensions());
}
