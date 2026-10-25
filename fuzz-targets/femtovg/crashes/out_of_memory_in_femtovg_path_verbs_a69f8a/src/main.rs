use femtovg::Path;

fn main() {
    let mut path = Path::default();
    path.rounded_rect(f32::MIN, 0.0f32, 0.0f32, 0.0f32, 1.0f32);
    let path2 = path.dashed(&[1.0f32], 0.0f32);
    let _ = path2.verbs();
}
