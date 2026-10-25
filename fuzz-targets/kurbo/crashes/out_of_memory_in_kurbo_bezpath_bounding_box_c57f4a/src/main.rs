use kurbo::BezPath;

fn main() {
    let _ = <kurbo::BezPath as kurbo::Shape>::bounding_box(&BezPath::with_capacity(1000000000usize));
}
