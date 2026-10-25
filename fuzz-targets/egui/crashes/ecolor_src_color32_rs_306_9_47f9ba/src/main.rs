use ecolor::Color32;

fn main() {
    let _ = Color32::from_rgba_unmultiplied(0u8, 0u8, 0u8, 0u8).linear_multiply(f32::MIN);
}
