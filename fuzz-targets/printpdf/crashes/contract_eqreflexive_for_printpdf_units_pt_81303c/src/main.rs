use printpdf::units::Pt;
use printpdf::units::Px;

fn main() {
    let px = <printpdf::units::Px as ::core::ops::Sub<printpdf::units::Px>>::sub(Px(0usize), Pt::default().into_px(0.0f32));
    let pt = px.into_pt(0.0f32);
    assert!(pt == pt, "Eq is not reflexive");
}
