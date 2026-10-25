use krilla::color::luma::Color;
use krilla::num::NormalizedF32;
use krilla::paint::LineCap;
use krilla::paint::LineJoin;
use krilla::paint::Paint;
use krilla::paint::Stroke;
use krilla::paint::StrokeDash;

fn main() {
    let stroke = Stroke { paint: Paint::from(Color::new(1u8)), width: 0.0f32, miter_limit: 0.0f32, line_cap: LineCap::Round, line_join: LineJoin::default(), opacity: NormalizedF32::ZERO, dash: Some(StrokeDash { array: vec![f32::NAN], offset: 1.0f32 }) };
    assert!(stroke == stroke, "Eq is not reflexive");
}
