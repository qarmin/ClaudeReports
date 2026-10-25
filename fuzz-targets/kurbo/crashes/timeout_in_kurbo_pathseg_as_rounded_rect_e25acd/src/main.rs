use kurbo::Triangle;

fn main() {
    let triangle = Triangle::EQUILATERAL;
    let circle = triangle.inscribed_circle();
    let _ = <kurbo::PathSeg as kurbo::Shape>::as_rounded_rect(&<kurbo::Arc as kurbo::Shape>::to_path(&circle.segment(0.0f64, 571.0f64, 10000000000.0f64).inner_arc(), -2.6531947294924042e60f64).get_seg(0usize).unwrap());
}
