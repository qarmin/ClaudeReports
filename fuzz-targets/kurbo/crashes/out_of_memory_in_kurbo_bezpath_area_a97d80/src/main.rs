use kurbo::BezPath;

fn main() {
    let _ = <kurbo::BezPath as kurbo::Shape>::area(&BezPath::with_capacity(1000000000usize));
}
