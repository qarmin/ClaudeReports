use glow::ShaderPrecisionFormat;

fn main() {
    let _ = ShaderPrecisionFormat::common_desktop_hardware(0u32, false);
}
