use image::{imageops::FilterType, DynamicImage, ImageBuffer, Luma};

fn run(src_w: u32, dst_w: u32) {
    let src: ImageBuffer<Luma<f32>, Vec<f32>> = ImageBuffer::from_fn(src_w, 1, |x, _y| {
        Luma([x as f32 / (src_w - 1) as f32])
    });
    let mut dyn_img = DynamicImage::ImageLuma32F(src);
    dyn_img.resize_exact(dst_w, 1, FilterType::Gaussian);
    let out = dyn_img.to_luma32f();

    let scale = src_w as f32 / dst_w as f32;
    let margin = (dst_w / 10).max(3);
    let mut max_dev = 0f32;
    let mut max_i = 0u32;
    for i in margin..(dst_w - margin) {
        let center = (i as f32 + 0.5) * scale - 0.5;
        if center < 0.0 || center > (src_w - 1) as f32 {
            continue;
        }
        let expected = center / (src_w - 1) as f32;
        let actual = out.get_pixel(i, 0).0[0];
        let dev = (actual - expected).abs();
        if dev > max_dev {
            max_dev = dev;
            max_i = i;
        }
    }
    println!("src_w={src_w} dst_w={dst_w}: max_dev={max_dev:.5} at i={max_i}");
}

fn main() {
    for (src_w, dst_w) in [(5u32, 40u32), (5, 100), (5, 750), (10, 80), (10, 400), (10, 1500)] {
        run(src_w, dst_w);
    }
}
