use kurbo::BezPath;

fn main() {
    let _ = <kurbo::PathSeg as kurbo::Shape>::path_segments(&BezPath::with_capacity(1000000000usize).get_seg(0usize).unwrap(), 0.0f64);
}
