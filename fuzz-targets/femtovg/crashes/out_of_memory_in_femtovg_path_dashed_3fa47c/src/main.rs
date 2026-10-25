use femtovg::FontMetrics;
use femtovg::Path;

fn main() {
    let mut path = Path::new();
    let font_metrics = FontMetrics::default();
    let f32_v = font_metrics.height();
    path.svg_arc_to(1.0f32, f32::MIN, 1.0f32, false, false, f32_v, 1.0f32);
    let _ = path.dashed(&[1.0f32], 0.0f32);
}
