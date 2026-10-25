use kurbo::BezPath;

fn main() {
    let _ = <kurbo::RoundedRect as kurbo::Shape>::as_circle(&<kurbo::PathSeg as kurbo::Shape>::as_rounded_rect(&BezPath::with_capacity(1000000000usize).get_seg(0usize).unwrap()).unwrap());
}
