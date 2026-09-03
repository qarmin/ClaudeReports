// Checks whether the "uniform input stays uniform" invariant (documented and enforced for
// `imageops::resize`'s `vertical_sample` path by candidate 003/009's regression test) also holds
// for the *separate*, `pic-scale-safe`-backed `DynamicImage::resize`/`resize_exact`/
// `resize_to_fill` methods, and for `imageops::thumbnail`/`DynamicImage::thumbnail_exact` (a third,
// independent implementation). None of these currently have a dedicated regression test for this
// invariant in the crate's own test suite.
//
// Every filter here is documented to be a normalized (sum-to-1.0) weighted average, so any
// mis-seeded accumulator/off-by-one bias (the class candidate 003 found in `vertical_sample`)
// would show up immediately as a uniform-color-input violation, exactly like it did there.

use image::imageops::FilterType;
use image::{DynamicImage, ImageBuffer, LumaA, Rgba};

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
    // (src_w, src_h, dst_w, dst_h) - upscale, downscale, extreme aspect ratio, 1-pixel source
    &[
        (6, 6, 3, 3),
        (5, 5, 11, 11),
        (7, 3, 3, 7),
        (1, 1, 4, 4),
        (4, 4, 1, 1),
        (2, 200, 50, 50),
    ]
}

fn main() {
    let mut mismatches: Vec<String> = Vec::new();

    // u8 RGBA, opaque and partially-transparent (exercises the premultiply/unpremultiply path)
    for &filter in &filters() {
        for &(sw, sh, dw, dh) in sizes() {
            for &color in &[Rgba([123u8, 45, 200, 255]), Rgba([123u8, 45, 200, 128])] {
                let src: ImageBuffer<Rgba<u8>, Vec<u8>> = ImageBuffer::from_pixel(sw, sh, color);

                let mut resized = DynamicImage::ImageRgba8(src.clone());
                resized.resize_exact(dw, dh, filter);
                check_rgba8(&resized, color, &mut mismatches, "resize_exact", filter, sw, sh, dw, dh);

                let mut resized_ar = DynamicImage::ImageRgba8(src.clone());
                resized_ar.resize(dw, dh, filter);
                check_rgba8(&resized_ar, color, &mut mismatches, "resize(aspect)", filter, sw, sh, dw, dh);

                let mut filled = DynamicImage::ImageRgba8(src.clone());
                filled.resize_to_fill(dw, dh, filter);
                check_rgba8(&filled, color, &mut mismatches, "resize_to_fill", filter, sw, sh, dw, dh);

                let thumb = image::imageops::thumbnail(&src, dw, dh);
                for (x, y, p) in thumb.enumerate_pixels() {
                    if *p != color {
                        mismatches.push(format!(
                            "thumbnail color={color:?} src=({sw}x{sh}) dst=({dw}x{dh}) pixel=({x},{y}) got={p:?}"
                        ));
                    }
                }

                let thumb_exact = DynamicImage::ImageRgba8(src.clone()).thumbnail_exact(dw, dh);
                check_rgba8(&thumb_exact, color, &mut mismatches, "thumbnail_exact", filter, sw, sh, dw, dh);
            }
        }
    }

    // f32 (unclamped - candidate 003's RESULT.md flagged this pixel type as affected
    // unconditionally by that bug class, with no u8 saturation to mask it) and u16
    let color_f32 = Rgba([0.3f32, 0.6, 0.9, 0.75]);
    let color_u16 = Rgba([12345u16, 6789, 40000, 65535]);
    for &filter in &filters() {
        for &(sw, sh, dw, dh) in sizes() {
            let src_f32: ImageBuffer<Rgba<f32>, Vec<f32>> = ImageBuffer::from_pixel(sw, sh, color_f32);
            let mut resized = DynamicImage::ImageRgba32F(src_f32);
            resized.resize_exact(dw, dh, filter);
            if let DynamicImage::ImageRgba32F(buf) = &resized {
                for (i, chunk) in buf.as_raw().chunks_exact(4).enumerate() {
                    let close = chunk.iter().zip(color_f32.0.iter()).all(|(a, b)| (a - b).abs() < 1e-4);
                    if !close {
                        mismatches.push(format!(
                            "f32 resize_exact filter={filter:?} src=({sw}x{sh}) dst=({dw}x{dh}) idx={i} got={chunk:?}"
                        ));
                    }
                }
            }

            let src_u16: ImageBuffer<Rgba<u16>, Vec<u16>> = ImageBuffer::from_pixel(sw, sh, color_u16);
            let mut resized16 = DynamicImage::ImageRgba16(src_u16);
            resized16.resize_exact(dw, dh, filter);
            if let DynamicImage::ImageRgba16(buf) = &resized16 {
                for (i, chunk) in buf.as_raw().chunks_exact(4).enumerate() {
                    if chunk != color_u16.0 {
                        mismatches.push(format!(
                            "u16 resize_exact filter={filter:?} src=({sw}x{sh}) dst=({dw}x{dh}) idx={i} got={chunk:?}"
                        ));
                    }
                }
            }
        }
    }

    // LumaA8 (single-channel-plus-alpha, a different has_constant_alpha branch than RGBA)
    let color_luma = LumaA([200u8, 30]);
    for &filter in &filters() {
        for &(sw, sh, dw, dh) in sizes() {
            let src: ImageBuffer<LumaA<u8>, Vec<u8>> = ImageBuffer::from_pixel(sw, sh, color_luma);
            let mut resized = DynamicImage::ImageLumaA8(src.clone());
            resized.resize_exact(dw, dh, filter);
            if let DynamicImage::ImageLumaA8(buf) = &resized {
                for (i, chunk) in buf.as_raw().chunks_exact(2).enumerate() {
                    if chunk != color_luma.0 {
                        mismatches.push(format!(
                            "LumaA8 resize_exact filter={filter:?} src=({sw}x{sh}) dst=({dw}x{dh}) idx={i} got={chunk:?}"
                        ));
                    }
                }
            }

            let thumb = image::imageops::thumbnail(&src, dw, dh);
            for (i, chunk) in thumb.as_raw().chunks_exact(2).enumerate() {
                if chunk != color_luma.0 {
                    mismatches.push(format!(
                        "LumaA8 thumbnail src=({sw}x{sh}) dst=({dw}x{dh}) idx={i} got={chunk:?}"
                    ));
                }
            }
        }
    }

    println!("{} size/filter/color combinations checked", filters().len() * sizes().len());
    if mismatches.is_empty() {
        println!("Not reproduced: uniform-color input stayed uniform through resize/resize_exact/resize/resize_to_fill/thumbnail/thumbnail_exact for u8 RGBA (opaque+alpha), f32, u16 and LumaA8");
    } else {
        println!("BUG REPRODUCED: {} mismatches (showing up to 20)", mismatches.len());
        for m in mismatches.iter().take(20) {
            println!("  {m}");
        }
        std::process::exit(1);
    }
}

fn check_rgba8(
    img: &DynamicImage,
    expected: Rgba<u8>,
    mismatches: &mut Vec<String>,
    label: &str,
    filter: FilterType,
    sw: u32,
    sh: u32,
    dw: u32,
    dh: u32,
) {
    let rgba = img.to_rgba8();
    for (x, y, p) in rgba.enumerate_pixels() {
        if *p != expected {
            mismatches.push(format!(
                "{label} filter={filter:?} color={expected:?} src=({sw}x{sh}) dst=({dw}x{dh}) pixel=({x},{y}) got={p:?}"
            ));
        }
    }
}
