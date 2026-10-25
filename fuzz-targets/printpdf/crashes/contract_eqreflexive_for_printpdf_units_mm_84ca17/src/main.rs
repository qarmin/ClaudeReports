use printpdf::units::Mm;

fn main() {
    let mm = Mm(f32::NAN);
    assert!(mm == mm, "Eq is not reflexive");
}
