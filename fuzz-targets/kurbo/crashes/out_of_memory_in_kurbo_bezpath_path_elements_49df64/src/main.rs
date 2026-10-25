use kurbo::BezPath;

fn main() {
    let _ = <kurbo::BezPath as kurbo::Shape>::path_elements(&BezPath::with_capacity(1000000000usize), 0.0f64);
}
