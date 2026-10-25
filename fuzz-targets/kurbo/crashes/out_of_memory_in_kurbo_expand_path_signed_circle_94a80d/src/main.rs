use kurbo::Diagonal2;
use kurbo::Join;
use kurbo::Triangle;

fn main() {
    let _ = kurbo::expand_path_signed(Triangle::EQUILATERAL.circumscribed_circle(), <kurbo::Diagonal2 as ::core::ops::Neg>::neg(Diagonal2 { xx: 1.0f64, yy: -1.2817015970884436e126f64 }), Join::Round, 0.0f64, 1.0f64);
}
