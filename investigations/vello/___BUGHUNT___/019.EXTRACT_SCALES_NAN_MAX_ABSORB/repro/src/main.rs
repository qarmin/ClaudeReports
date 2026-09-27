//! Reproduces vello_common::util::extract_scales silently absorbing a NaN scale_y
//! (caused by floating-point rounding making `s1 - s2` slightly negative for a
//! near-singular affine transform) into the arbitrary floor value 1e-6, via f32::max's
//! NaN-ignoring semantics.
//!
//! Drives the crate's real public API (`vello_common::kurbo::Affine` + the public
//! `vello_common::util::extract_scales` function) - this is a pure numerical function
//! over a single caller-supplied value (an affine transform), the same kind of "one
//! external artifact handed to a public entry point" case CONFIRMATION.md section 11
//! says needs no second participant to fake.
//!
//! Exits 0 (bug fixed) if every case's `scale_y` differs from the arbitrary 1e-6 floor
//! by more than a small tolerance; exits 1 (bug present) otherwise, printing which
//! case(s) hit the floor.

use vello_common::kurbo::Affine;
use vello_common::util::extract_scales;

fn main() {
    // Near-singular affine transforms - the kind of matrix a real caller can produce
    // via an ordinary combination of scale/rotate/skew calls, or by composing
    // user-supplied transforms. Coefficients found by a randomized search over exactly
    // this formula (see the candidate's own RESULT.md for the search and the f64
    // "ground truth" scale_y each one should actually produce).
    let cases: [[f64; 4]; 3] = [
        [0.5923988, 0.51015484, 0.68755627, 0.59199905],
        [0.8449424, 0.85955596, 0.83047223, 0.8449993],
        [-0.5948196, 0.4599222, 0.76916146, -0.5949993],
    ];

    let mut bug_present = false;

    for [a, b, c, d] in cases {
        let transform = Affine::new([a, b, c, d, 0.0, 0.0]);
        let (scale_x, scale_y) = extract_scales(&transform);
        let hit_floor = (scale_y - 1e-6).abs() < 1e-9;
        println!(
            "a={a} b={b} c={c} d={d} -> scale_x={scale_x} scale_y={scale_y}{}",
            if hit_floor {
                " <- hit the arbitrary 1e-6 floor (BUG)"
            } else {
                " (not floored - a small, transform-specific value)"
            }
        );
        if hit_floor {
            bug_present = true;
        }
    }

    if bug_present {
        println!(
            "\nBUG PRESENT: at least one near-singular transform above collapsed to the \
             arbitrary 1e-6 floor via NaN silently absorbed by f32::max, instead of the small \
             but transform-specific scale_y it should have produced."
        );
        std::process::exit(1);
    } else {
        println!("\nBUG FIXED: every case produced a transform-specific scale_y, none hit the arbitrary 1e-6 floor.");
    }
}
