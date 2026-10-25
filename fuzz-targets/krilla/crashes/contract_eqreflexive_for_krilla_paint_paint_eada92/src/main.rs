use krilla::geom::Transform;
use krilla::paint::LinearGradient;
use krilla::paint::Paint;
use krilla::paint::SpreadMethod;

fn main() {
    let paint = Paint::from(LinearGradient { x1: 0.0f32, y1: 0.0f32, x2: 0.0f32, y2: 0.0f32, transform: Transform::from_scale(0.0f32, 0.0f32).invert().unwrap(), spread_method: SpreadMethod::Pad, stops: vec![], anti_alias: false });
    assert!(paint == paint, "Eq is not reflexive");
}
