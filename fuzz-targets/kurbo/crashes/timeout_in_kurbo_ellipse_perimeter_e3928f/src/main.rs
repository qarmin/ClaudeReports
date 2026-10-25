use kurbo::Ellipse;

fn main() {
    let _ = <kurbo::Ellipse as kurbo::Shape>::perimeter(&Ellipse::default(), f64::MIN);
}
