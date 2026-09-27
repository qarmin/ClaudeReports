// Follow-up to candidate 010 (RESIZE_THUMBNAIL_UNIFORM_COLOR_TEST_GAP), which swept
// DynamicImage::resize/resize_exact/resize_to_fill/imageops::thumbnail/thumbnail_exact for
// uniform-color-input preservation across u8 RGBA (opaque+alpha)/f32/u16 RGBA/LumaA8 without
// finding a live bug, but explicitly left two things unchecked (see its RESULT.md "What's not
// done"): the no-alpha color types (Luma8/Luma16/Rgb16/Luma32F/Rgb32F - the trivial
// has_constant_alpha=>true branch, never premultiplied) and a randomized (rather than fixed-grid)
// size/filter/color sweep. This repro closes both gaps against the real public API.

use image::imageops::FilterType;
use image::{DynamicImage, ImageBuffer, Luma, Rgb, Rgba};

fn filters() -> [FilterType; 5] {
    [
        FilterType::Nearest,
        FilterType::Triangle,
        FilterType::CatmullRom,
        FilterType::Gaussian,
        FilterType::Lanczos3,
    ]
}

fn sizes() -> &'static [(u32, u32, u32, u32)] {
    // (src_w, src_h, dst_w, dst_h)
    &[
        (8, 8, 16, 16),
        (16, 16, 8, 8),
        (200, 2, 50, 50),
        (1, 1, 4, 4),
        (4, 4, 1, 1),
        (7, 13, 13, 7),
    ]
}

// xorshift32 - no external RNG dependency needed for a scratch probe.
struct Xorshift32(u32);
impl Xorshift32 {
    fn next(&mut self) -> u32 {
        self.0 ^= self.0 << 13;
        self.0 ^= self.0 >> 17;
        self.0 ^= self.0 << 5;
        self.0
    }
}

fn main() {
    let mut mismatches: Vec<String> = Vec::new();

    // Fixed-grid sweep of the no-alpha color types candidate 010 left unchecked.
    for &filter in &filters() {
        for &(sw, sh, dw, dh) in sizes() {
            let src: ImageBuffer<Luma<u8>, Vec<u8>> = ImageBuffer::from_pixel(sw, sh, Luma([200u8]));
            let mut resized = DynamicImage::ImageLuma8(src);
            resized.resize_exact(dw, dh, filter);
            if let DynamicImage::ImageLuma8(buf) = &resized {
                for (x, y, p) in buf.enumerate_pixels() {
                    if p.0[0] != 200u8 {
                        mismatches.push(format!(
                            "Luma8 filter={filter:?} src=({sw}x{sh}) dst=({dw}x{dh}) pixel=({x},{y}) got={p:?}"
                        ));
                    }
                }
            }

            let src16: ImageBuffer<Luma<u16>, Vec<u16>> = ImageBuffer::from_pixel(sw, sh, Luma([40000u16]));
            let mut resized16 = DynamicImage::ImageLuma16(src16);
            resized16.resize_exact(dw, dh, filter);
            if let DynamicImage::ImageLuma16(buf) = &resized16 {
                for (x, y, p) in buf.enumerate_pixels() {
                    if p.0[0] != 40000u16 {
                        mismatches.push(format!(
                            "Luma16 filter={filter:?} src=({sw}x{sh}) dst=({dw}x{dh}) pixel=({x},{y}) got={p:?}"
                        ));
                    }
                }
            }

            let color_rgb16 = Rgb([1000u16, 2000u16, 3000u16]);
            let src_rgb16: ImageBuffer<Rgb<u16>, Vec<u16>> = ImageBuffer::from_pixel(sw, sh, color_rgb16);
            let mut resized_rgb16 = DynamicImage::ImageRgb16(src_rgb16);
            resized_rgb16.resize_exact(dw, dh, filter);
            if let DynamicImage::ImageRgb16(buf) = &resized_rgb16 {
                for (x, y, p) in buf.enumerate_pixels() {
                    if *p != color_rgb16 {
                        mismatches.push(format!(
                            "Rgb16 filter={filter:?} src=({sw}x{sh}) dst=({dw}x{dh}) pixel=({x},{y}) got={p:?}"
                        ));
                    }
                }
            }

            let src_l32f: ImageBuffer<Luma<f32>, Vec<f32>> = ImageBuffer::from_pixel(sw, sh, Luma([0.37f32]));
            let mut resized_l32f = DynamicImage::ImageLuma32F(src_l32f);
            resized_l32f.resize_exact(dw, dh, filter);
            if let DynamicImage::ImageLuma32F(buf) = &resized_l32f {
                for (x, y, p) in buf.enumerate_pixels() {
                    if (p.0[0] - 0.37f32).abs() >= 1e-4 {
                        mismatches.push(format!(
                            "Luma32F filter={filter:?} src=({sw}x{sh}) dst=({dw}x{dh}) pixel=({x},{y}) got={p:?}"
                        ));
                    }
                }
            }

            let color_rgb32f = Rgb([0.1f32, 0.5, 0.9]);
            let src_rgb32f: ImageBuffer<Rgb<f32>, Vec<f32>> = ImageBuffer::from_pixel(sw, sh, color_rgb32f);
            let mut resized_rgb32f = DynamicImage::ImageRgb32F(src_rgb32f);
            resized_rgb32f.resize_exact(dw, dh, filter);
            if let DynamicImage::ImageRgb32F(buf) = &resized_rgb32f {
                for (x, y, p) in buf.enumerate_pixels() {
                    let close = p.0.iter().zip(color_rgb32f.0.iter()).all(|(a, b)| (a - b).abs() < 1e-4);
                    if !close {
                        mismatches.push(format!(
                            "Rgb32F filter={filter:?} src=({sw}x{sh}) dst=({dw}x{dh}) pixel=({x},{y}) got={p:?}"
                        ));
                    }
                }
            }
        }
    }

    // Randomized sweep (500 iterations) over sizes/filter/RGBA8 color - escalation beyond the
    // fixed grid above.
    let mut rng = Xorshift32(0xC0FFEE42);
    for _ in 0..500 {
        let sw = 1 + rng.next() % 40;
        let sh = 1 + rng.next() % 40;
        let dw = 1 + rng.next() % 40;
        let dh = 1 + rng.next() % 40;
        let filter = filters()[(rng.next() % filters().len() as u32) as usize];
        let color = Rgba([
            (rng.next() % 256) as u8,
            (rng.next() % 256) as u8,
            (rng.next() % 256) as u8,
            (rng.next() % 256) as u8,
        ]);

        let src: ImageBuffer<Rgba<u8>, Vec<u8>> = ImageBuffer::from_pixel(sw, sh, color);
        let mut resized = DynamicImage::ImageRgba8(src);
        resized.resize_exact(dw, dh, filter);
        if let DynamicImage::ImageRgba8(buf) = &resized {
            for (x, y, p) in buf.enumerate_pixels() {
                if *p != color {
                    mismatches.push(format!(
                        "random Rgba8 filter={filter:?} color={color:?} src=({sw}x{sh}) dst=({dw}x{dh}) pixel=({x},{y}) got={p:?}"
                    ));
                }
            }
        }
    }

    println!(
        "{} fixed-grid combinations + 500 randomized combinations checked",
        filters().len() * sizes().len()
    );
    if mismatches.is_empty() {
        println!("Not reproduced: uniform-color input stayed uniform through resize_exact for Luma8/Luma16/Rgb16/Luma32F/Rgb32F (fixed grid) and randomized Rgba8 (500 iters)");
    } else {
        println!("BUG REPRODUCED: {} mismatches (showing up to 20)", mismatches.len());
        for m in mismatches.iter().take(20) {
            println!("  {m}");
        }
        std::process::exit(1);
    }
}
