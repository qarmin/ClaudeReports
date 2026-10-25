use kurbo::MinDistance;
use kurbo::RoundedRectRadii;
use kurbo::Size;
use kurbo::simplify::SimplifyOptLevel;
use kurbo::simplify::SimplifyOptions;

fn main() {
    let size = Size::default();
    let size2 = <kurbo::Size as ::core::ops::Add<kurbo::Size>>::add(size, <kurbo::Size as ::core::ops::Mul<f64>>::mul(Size::from((0.9999999999999999f64, -34.6f64)).expand(), -3.67e19f64).floor());
    let rounded_rect = size2.to_rounded_rect(RoundedRectRadii { top_left: 1e-310f64, top_right: 1.0f64, bottom_right: 0.5f64, bottom_left: 0.0f64 }.clamp(853.0f64).abs());
    let _ = MinDistance { distance: 65536.0f64, t1: 0.106f64, t2: 0.9999999999999999f64 };
    let simplify_opt_level = SimplifyOptLevel::Optimize;
    let _ = <kurbo::PathSeg as kurbo::Shape>::area(&<kurbo::RoundedRect as kurbo::Shape>::to_path(&rounded_rect, 0.0f64).get_seg(11usize).unwrap());
    let simplify_options = SimplifyOptions::default();
    let simplify_options2 = simplify_options.opt_level(simplify_opt_level);
    let _ = simplify_options2.angle_thresh(0.25000000000000006f64);
}
