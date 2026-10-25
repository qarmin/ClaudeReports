use krilla::geom::Transform;
use krilla::num::NormalizedF32;
use krilla::paint::Fill;
use krilla::paint::FillRule;
use krilla::paint::Paint;
use krilla::paint::SpreadMethod;
use krilla::paint::SweepGradient;

fn main() {
    let fill = Fill { paint: Paint::from(SweepGradient { cx: 0.0f32, cy: f32::NAN, start_angle: 0.0f32, end_angle: 0.0f32, transform: Transform::identity(), spread_method: SpreadMethod::default(), stops: vec![], anti_alias: false }), opacity: NormalizedF32::ZERO, rule: FillRule::NonZero };
    assert!(fill == fill, "Eq is not reflexive");
}
