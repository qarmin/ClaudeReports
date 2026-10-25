use kurbo::Circle;

fn main() {
    let circle = Circle::default();
    let _ = <kurbo::PathSeg as kurbo::Shape>::as_rounded_rect(&<kurbo::CircleSegment as kurbo::Shape>::to_path(&circle.segment(0.0f64, 0.0f64, f64::INFINITY), 0.0f64).get_seg(0usize).unwrap());
}
