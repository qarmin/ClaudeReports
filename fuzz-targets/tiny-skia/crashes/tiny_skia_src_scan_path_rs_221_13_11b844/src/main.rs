use tiny_skia::FillRule;
use tiny_skia::Mask;
use tiny_skia_path::PathBuilder;
use tiny_skia_path::Rect;
use tiny_skia_path::Transform;

fn main() {
    let transform = Transform::from_skew(1.0f32, 1.0f32);
    let transform2 = transform.pre_rotate(0.0f32);
    let bool_v = transform2.has_skew();
    let fill_rule = FillRule::default();
    let path = PathBuilder::from_rect(Rect::from_ltrb(-1e30f32, 0.0f32, 0.0f32, 1.0f32).unwrap());
    let mut mask = Mask::decode_png(&include_bytes!("data_0.bin").to_vec()).unwrap();
    mask.fill_path(&path, fill_rule, bool_v, transform);
}
