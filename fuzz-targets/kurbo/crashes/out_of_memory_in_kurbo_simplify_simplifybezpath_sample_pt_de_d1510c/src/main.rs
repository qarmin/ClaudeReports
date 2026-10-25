use kurbo::BezPath;
use kurbo::simplify::SimplifyBezPath;

fn main() {
    let _ = <kurbo::simplify::SimplifyBezPath as kurbo::ParamCurveFit>::sample_pt_deriv(&SimplifyBezPath::new(vec![BezPath::with_capacity(1000000000usize).pop().unwrap()]), 0.0f64);
}
