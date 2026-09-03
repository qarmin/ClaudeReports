use pic_scale_safe::{resize_plane_f32, ImageSize, ResamplingFunction};

fn ramp(src_w: usize) -> Vec<f32> {
    (0..src_w).map(|x| x as f32 / (src_w - 1) as f32).collect()
}

fn run(src_w: usize, dst_w: usize, f: ResamplingFunction, name: &str) {
    let src = ramp(src_w);
    let src_size = ImageSize::new(src_w, 1);
    let dst_size = ImageSize::new(dst_w, 1);
    let out = resize_plane_f32(&src, src_size, dst_size, f).unwrap();

    // Expected value for a perfectly reproduced linear ramp at output index i:
    // the source-space center position mapped back through the same (i+0.5)*scale-0.5 convention.
    let scale = src_w as f32 / dst_w as f32;
    let margin = (dst_w / 10).max(3);
    let mut max_dev = 0f32;
    let mut max_i = 0usize;
    for i in margin..(dst_w - margin) {
        let center = (i as f32 + 0.5) * scale - 0.5;
        if center < 0.0 || center > (src_w - 1) as f32 {
            continue;
        }
        let expected = center / (src_w - 1) as f32;
        let dev = (out[i] - expected).abs();
        if dev > max_dev {
            max_dev = dev;
            max_i = i;
        }
    }
    println!("{name}: src_w={src_w} dst_w={dst_w} max_dev={max_dev:.5} at i={max_i}");
}

fn main() {
    let cases = [(5usize, 40usize), (5, 100), (5, 750), (10, 80), (10, 400), (10, 1500)];
    for (src_w, dst_w) in cases {
        run(src_w, dst_w, ResamplingFunction::Gaussian, "Gaussian");
        run(src_w, dst_w, ResamplingFunction::CatmullRom, "CatmullRom");
    }
}
