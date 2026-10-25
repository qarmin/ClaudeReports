use kurbo::BezPath;

fn main() {
    let _ = <kurbo::CubicBez as kurbo::Shape>::to_path(&BezPath::with_capacity(100000000000usize).get_seg(0usize).unwrap().to_cubic(), 1.0f64);
}
