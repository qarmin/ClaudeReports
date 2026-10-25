use kurbo::simplify::SimplifyBezPath;

fn main() {
    let _ = <kurbo::simplify::SimplifyBezPath as kurbo::ParamCurveFit>::moment_integrals(&SimplifyBezPath::new(vec![]), f64::NAN..0.0f64);
}
