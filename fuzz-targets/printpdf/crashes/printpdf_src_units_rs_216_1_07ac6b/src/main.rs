use printpdf::units::Px;

fn main() {
    let _ = <printpdf::units::Px as ::core::ops::Add<printpdf::units::Px>>::add(Px(usize::MAX), Px(1usize));
}
