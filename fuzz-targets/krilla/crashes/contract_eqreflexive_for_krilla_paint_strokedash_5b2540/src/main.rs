use krilla::paint::StrokeDash;

fn main() {
    let stroke_dash = StrokeDash { array: vec![f32::NAN], offset: 0.0f32 };
    assert!(stroke_dash == stroke_dash, "Eq is not reflexive");
}
