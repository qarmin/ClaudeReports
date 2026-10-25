use krilla::geom::Transform;
use krilla::paint::LinearGradient;
use krilla::paint::SpreadMethod;

fn main() {
    let linear_gradient = LinearGradient { x1: 0.0f32, y1: 0.0f32, x2: 0.0f32, y2: 1.0f32, transform: Transform::from_skew(0.0f32, f32::NAN), spread_method: SpreadMethod::default(), stops: vec![], anti_alias: false };
    assert!(linear_gradient == linear_gradient, "Eq is not reflexive");
}
