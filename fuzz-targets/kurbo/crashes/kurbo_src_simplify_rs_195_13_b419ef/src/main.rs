use kurbo::simplify::SimplifyBezPath;

fn main() {
    let _ = <kurbo::simplify::SimplifyBezPath as kurbo::ParamCurveFit>::sample_pt_deriv(&SimplifyBezPath::new(vec![]), 0.0f64);
}
