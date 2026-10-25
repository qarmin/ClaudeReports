use kurbo::PathEl;
use kurbo::Point;
use kurbo::simplify::SimplifyBezPath;

fn main() {
    let simplify_bez_path = SimplifyBezPath::new(vec![PathEl::QuadTo(Point::default(), Point::ORIGIN)]);
    let _ = <kurbo::simplify::SimplifyBezPath as kurbo::ParamCurveFit>::sample_pt_deriv(&simplify_bez_path, 1.5763829091294793e257f64);
}
