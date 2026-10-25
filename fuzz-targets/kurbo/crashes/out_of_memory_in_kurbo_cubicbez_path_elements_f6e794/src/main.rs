use kurbo::BezPath;

fn main() {
    let _ = <kurbo::CubicBez as kurbo::Shape>::path_elements(&BezPath::with_capacity(1000000000usize).get_seg(0usize).unwrap().to_cubic(), 0.0f64);
}
