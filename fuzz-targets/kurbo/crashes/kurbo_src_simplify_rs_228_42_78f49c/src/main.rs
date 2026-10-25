use kurbo::PathEl;
use kurbo::Point;
use kurbo::simplify::SimplifyBezPath;

fn main() {
    let _ = <kurbo::simplify::SimplifyBezPath as kurbo::ParamCurveFit>::moment_integrals(&SimplifyBezPath::new(vec![PathEl::LineTo(Point::ORIGIN)]), 0.0f64..1e300f64);
}
