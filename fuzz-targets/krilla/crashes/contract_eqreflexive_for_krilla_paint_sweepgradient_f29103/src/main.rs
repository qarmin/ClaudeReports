use krilla::geom::Transform;
use krilla::paint::SpreadMethod;
use krilla::paint::SweepGradient;

fn main() {
    let sweep_gradient = SweepGradient { cx: 0.0f32, cy: 0.0f32, start_angle: f32::NAN, end_angle: 0.0f32, transform: Transform::from_rotate(0.0f32), spread_method: SpreadMethod::Repeat, stops: vec![], anti_alias: false };
    assert!(sweep_gradient == sweep_gradient, "Eq is not reflexive");
}
