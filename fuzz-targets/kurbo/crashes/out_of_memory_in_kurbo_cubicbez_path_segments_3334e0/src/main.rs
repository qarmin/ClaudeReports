use kurbo::BezPath;

fn main() {
    let _ = <kurbo::CubicBez as kurbo::Shape>::path_segments(&BezPath::with_capacity(67108863usize).get_seg(0usize).unwrap().to_cubic(), 0.0f64);
}
