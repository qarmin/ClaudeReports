use image::imageops::GaussianBlurParameters;

fn main() {
    let _ = GaussianBlurParameters::new_anisotropic_kernel_size(0.357f32, 1.0f32);
}
