use glow::MAX_TESS_CONTROL_IMAGE_UNIFORMS;

fn main() {
    let u32_v = MAX_TESS_CONTROL_IMAGE_UNIFORMS;
    let _ = glow::components_per_format(u32_v);
}
