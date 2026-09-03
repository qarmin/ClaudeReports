use image::{math::Rect, DynamicImage, GenericImageView, RgbImage};

fn old_resize_to_fill(img: &DynamicImage, nwidth: u32, nheight: u32, filter: image::imageops::FilterType) -> DynamicImage {
    // Re-implementation of the pre-patch algorithm: resize to the (possibly huge) fill
    // dimensions, then crop.
    let (width, height) = img.dimensions();
    let wratio = nwidth as f64 / width as f64;
    let hratio = nheight as f64 / height as f64;
    let ratio = f64::max(wratio, hratio);
    let width2 = ((width as f64 * ratio).round() as u32).max(1);
    let height2 = ((height as f64 * ratio).round() as u32).max(1);

    let mut resized = img.clone();
    resized.resize_exact(width2, height2, filter);

    let (iwidth, iheight) = resized.dimensions();
    let r = u64::from(iwidth) * u64::from(nheight);
    let nr = u64::from(nwidth) * u64::from(iheight);
    let select = if nr > r {
        let y = (iheight - nheight) / 2;
        Rect::from_xy_ranges(0..nwidth, y..y + nheight)
    } else {
        let x = (iwidth - nwidth) / 2;
        Rect::from_xy_ranges(x..x + nwidth, 0..nheight)
    };
    resized.crop(select)
}

fn main() {
    let cases: &[(u32, u32, u32, u32)] = &[
        (800, 600, 100, 50),
        (800, 600, 50, 100),
        (100, 100, 40, 40),
        (37, 91, 20, 20),
        (640, 480, 480, 640),
        (300, 300, 300, 300),
    ];
    let mut any_fail = false;
    for &(w, h, nw, nh) in cases {
        let mut src = RgbImage::new(w, h);
        for (x, y, px) in src.enumerate_pixels_mut() {
            let r = (x as f64 / w.max(1) as f64 * 255.0) as u8;
            let g = (y as f64 / h.max(1) as f64 * 255.0) as u8;
            let b = ((x + y) as f64 / (w + h).max(1) as f64 * 255.0) as u8;
            *px = image::Rgb([r, g, b]);
        }
        let src = DynamicImage::ImageRgb8(src);

        let old = old_resize_to_fill(&src, nw, nh, image::imageops::FilterType::Triangle);
        let mut newv = src.clone();
        newv.resize_to_fill(nw, nh, image::imageops::FilterType::Triangle);

        let old_rgb = old.to_rgb8();
        let new_rgb = newv.to_rgb8();

        if old_rgb.dimensions() != new_rgb.dimensions() {
            println!("DIM MISMATCH for {:?}: old={:?} new={:?}", (w, h, nw, nh), old_rgb.dimensions(), new_rgb.dimensions());
            any_fail = true;
            continue;
        }

        let mut max_diff: i32 = 0;
        let mut sum_diff: i64 = 0;
        let n = old_rgb.pixels().len();
        for (a, b) in old_rgb.pixels().iter().zip(new_rgb.pixels().iter()) {
            for c in 0..3 {
                let d = (a[c] as i32 - b[c] as i32).abs();
                max_diff = max_diff.max(d);
                sum_diff += d as i64;
            }
        }
        let avg = sum_diff as f64 / (n as f64 * 3.0);
        println!("case {:?}: dims ok {:?}, max_channel_diff={}, avg_channel_diff={:.3}", (w, h, nw, nh), old_rgb.dimensions(), max_diff, avg);
    }
    if any_fail {
        std::process::exit(1);
    }
}
