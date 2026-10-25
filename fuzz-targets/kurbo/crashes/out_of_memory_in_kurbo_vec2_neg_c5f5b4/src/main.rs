use kurbo::BezPath;
use kurbo::Point;

fn main() {
    let _ = <kurbo::Vec2 as ::core::ops::Neg>::neg(<kurbo::Point as ::core::ops::Sub<kurbo::Point>>::sub(BezPath::with_capacity(100000000000usize).current_position().unwrap(), Point::ZERO));
}
