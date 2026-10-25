use kurbo::Rect;

fn main() {
    let bez_path = <kurbo::Rect as kurbo::Shape>::into_path(Rect::new(0.0f64, f64::MIN, 1.0f64, 0.0f64), 1.0f64);
    let path_seg = bez_path.get_seg(1usize).unwrap();
    let path_seg2 = path_seg.reverse();
    let cubic_bez = path_seg2.to_cubic();
    let _ = cubic_bez.approx_spline(0.0f64);
}
