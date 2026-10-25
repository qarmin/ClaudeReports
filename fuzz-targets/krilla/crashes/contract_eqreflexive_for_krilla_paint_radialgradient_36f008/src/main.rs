use krilla::geom::Transform;
use krilla::paint::RadialGradient;
use krilla::paint::SpreadMethod;

fn main() {
    let radial_gradient = RadialGradient { fx: 0.0f32, fy: 0.0f32, fr: 0.0f32, cx: f32::NAN, cy: 0.0f32, cr: 0.0f32, transform: Transform::identity(), spread_method: SpreadMethod::Pad, stops: vec![], anti_alias: false };
    assert!(radial_gradient == radial_gradient, "Eq is not reflexive");
}
