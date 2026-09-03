// Standalone reproduction for candidate 056: Affine::svd (and everything built on it -
// Affine::spectral_norm, Ellipse::radii/x_radius/y_radius/rotation/with_radii/with_rotation)
// squares (a+d) and (b-c) (and (a-d)/(b+c)) via `.powi(2)` *before* taking the square root, so a
// large-but-perfectly-representable uniform scale (e.g. a = d = 1e200, well under f64::MAX ~=
// 1.8e308) overflows to `inf` at the intermediate squaring step, long before the true
// mathematical result (which is also representable) would ever overflow.
//
// This drives the real, public `Affine::spectral_norm()` and `Ellipse::radii()` entry points
// directly - both are pure single-participant numeric computations on caller-supplied
// coefficients (there is no second participant/peer to fake here: the large-magnitude matrix
// coefficients themselves *are* the genuine external input a real caller would supply, the same
// way a malformed file is the genuine external input to a parser).
//
// Exit code convention: 0 = did not reproduce (bug absent), non-zero = reproduced (bug present).

use kurbo::{Affine, Ellipse};

fn main() {
    let mut reproduced = false;

    // 1. Affine::spectral_norm() on a large uniform scale: the true spectral norm is exactly
    // 1e200 (perfectly representable, f64::MAX ~= 1.7977e308), but the buggy code squares
    // (a+d) = 2e200 to 4e400 first, which overflows to `inf`.
    let sn = Affine::new([1e200, 0.0, 0.0, 1e200, 0.0, 0.0]).spectral_norm();
    println!("Affine::new([1e200,0,0,1e200,0,0]).spectral_norm() = {sn}");
    if !sn.is_finite() {
        eprintln!("REPRODUCED: spectral_norm() returned {sn} instead of a finite value (1e200)");
        reproduced = true;
    } else if (sn - 1e200).abs() > 1e192 {
        eprintln!("WRONG OUTPUT: spectral_norm() returned {sn}, expected ~1e200");
        reproduced = true;
    }

    // 2. Ellipse::radii() on a large-radii ellipse: built directly on Affine::svd() through the
    // same code path, so it inherits the same premature overflow.
    let radii = Ellipse::new((0.0, 0.0), (1e200, 1e200), 0.0).radii();
    println!("Ellipse::new((0,0),(1e200,1e200),0.0).radii() = {radii:?}");
    if !radii.x.is_finite() || !radii.y.is_finite() {
        eprintln!("REPRODUCED: radii() returned {radii:?} instead of finite (1e200, 1e200)");
        reproduced = true;
    }

    // 3. Boundary sweep: show the premature-overflow point sits far below f64::MAX, i.e. this is
    // not simply "the value is genuinely too large to represent".
    for exp in [150_i32, 153, 154, 155, 200, 300] {
        let v: f64 = format!("1e{exp}").parse().unwrap();
        let sn = Affine::new([v, 0.0, 0.0, v, 0.0, 0.0]).spectral_norm();
        println!("1e{exp} -> spectral_norm() = {sn} (finite: {})", sn.is_finite());
    }
    println!("f64::MAX = {}", f64::MAX);

    if reproduced {
        std::process::exit(1);
    } else {
        println!("did not reproduce - both spectral_norm() and radii() stayed finite");
    }
}
