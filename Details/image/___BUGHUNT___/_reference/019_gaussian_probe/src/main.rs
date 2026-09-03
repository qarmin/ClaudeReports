use image::{imageops, imageops::FilterType, DynamicImage, ImageBuffer, Luma};

struct Case { src_w: u32, dst_w: u32 }

fn run_case(case: &Case) {
    let Case { src_w, dst_w } = *case;
    let src: ImageBuffer<Luma<u8>, Vec<u8>> = ImageBuffer::from_fn(src_w, 1, |x, _y| {
        Luma([(x as f64 * 255.0 / (src_w - 1) as f64).round() as u8])
    });

    let reference = imageops::resize(&src, dst_w, 1, FilterType::Gaussian);

    let mut dyn_img = DynamicImage::ImageLuma8(src);
    dyn_img.resize_exact(dst_w, 1, FilterType::Gaussian);
    let subject = dyn_img.to_luma8();

    let margin = (dst_w / 20).max(2);
    let mut max_diff = 0i32;
    let mut max_diff_i = 0u32;
    for i in margin..(dst_w - margin) {
        let r = reference.get_pixel(i, 0).0[0] as i32;
        let s = subject.get_pixel(i, 0).0[0] as i32;
        let diff = (r - s).abs();
        if diff > max_diff { max_diff = diff; max_diff_i = i; }
    }
    println!("src_w={src_w} dst_w={dst_w}: max |reference(oracle Gaussian) - subject(pic-scale-safe Gaussian)| = {max_diff} at idx {max_diff_i}");
}

fn main() {
    let cases = [
        Case { src_w: 5, dst_w: 40 },
        Case { src_w: 5, dst_w: 100 },
        Case { src_w: 5, dst_w: 400 },
        Case { src_w: 5, dst_w: 750 },
        Case { src_w: 10, dst_w: 80 },
        Case { src_w: 10, dst_w: 400 },
        Case { src_w: 10, dst_w: 1500 },
    ];
    for case in &cases {
        run_case(case);
    }
}
