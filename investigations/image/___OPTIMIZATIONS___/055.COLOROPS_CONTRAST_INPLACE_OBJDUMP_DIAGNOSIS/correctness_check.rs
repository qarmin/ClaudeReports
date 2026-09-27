use image::math::Rect;
use image::{imageops, GenericImage, ImageBuffer, Rgb, Rgba};

fn fnv1a(bytes: &[u8]) -> u64 {
    let mut hash: u64 = 0xcbf29ce484222325;
    for &b in bytes {
        hash ^= b as u64;
        hash = hash.wrapping_mul(0x100000001b3);
    }
    hash
}

fn main() {
    let mut any_mismatch = false;

    // Rgb<u8> plain ImageBuffer
    let mut img: ImageBuffer<Rgb<u8>, Vec<u8>> = ImageBuffer::from_fn(257, 131, |x, y| {
        Rgb([x as u8, (y / 2) as u8, ((x * 7 + y) % 256) as u8])
    });
    imageops::contrast_in_place(&mut img, 5.0);
    println!("rgb plain: {:016x}", fnv1a(img.as_raw()));

    // Rgba<u8> plain ImageBuffer
    let mut img: ImageBuffer<Rgba<u8>, Vec<u8>> = ImageBuffer::from_fn(257, 131, |x, y| {
        Rgba([x as u8, (y / 2) as u8, ((x * 7 + y) % 256) as u8, 200])
    });
    imageops::contrast_in_place(&mut img, -20.0);
    println!("rgba plain: {:016x}", fnv1a(img.as_raw()));

    // Negative contrast
    let mut img: ImageBuffer<Rgb<u8>, Vec<u8>> = ImageBuffer::from_fn(64, 64, |x, y| {
        Rgb([x as u8, y as u8, 128])
    });
    imageops::contrast_in_place(&mut img, -500.0);
    println!("rgb extreme neg: {:016x}", fnv1a(img.as_raw()));

    // Large contrast (upper clamp)
    let mut img: ImageBuffer<Rgb<u8>, Vec<u8>> = ImageBuffer::from_fn(64, 64, |x, y| {
        Rgb([x as u8, y as u8, 128])
    });
    imageops::contrast_in_place(&mut img, 500.0);
    println!("rgb extreme pos: {:016x}", fnv1a(img.as_raw()));

    // SubImage fallback path
    let mut parent: ImageBuffer<Rgb<u8>, Vec<u8>> = ImageBuffer::from_fn(100, 100, |x, y| {
        Rgb([x as u8, y as u8, ((x + y) % 256) as u8])
    });
    {
        let mut sub = parent.sub_image(Rect {
            x: 10,
            y: 10,
            width: 50,
            height: 50,
        });
        imageops::contrast_in_place(&mut *sub, 5.0);
    }
    println!("rgb subimage: {:016x}", fnv1a(parent.as_raw()));

    let _ = any_mismatch;
}
