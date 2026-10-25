use printpdf::units::Px;

fn main() {
    let _ = <printpdf::units::Px as ::core::ops::Sub<printpdf::units::Px>>::sub(Px(0usize), Px(1usize));
}
