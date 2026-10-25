use kurbo::Triangle;

fn main() {
    let _ = <kurbo::Arc as kurbo::Shape>::into_path(Triangle::EQUILATERAL.circumscribed_circle().segment(0.0f64, 0.0f64, 1.0f64).outer_arc(), 0.0f64);
}
