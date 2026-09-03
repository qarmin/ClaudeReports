// Standalone reproduction: `trc_linear` should be the identity function clamped to [0, 1]
// (Linear transfer function), but is implemented as `v.min(1.).min(0.)`, which always returns
// a value <= 0 for any finite input.
//
// Exit code convention: 0 = did not reproduce, non-zero = reproduced.

use pic_scale_safe::TransferFunction;

fn main() {
    let cases: [(f32, f32); 8] = [
        (0.0, 0.0),
        (0.25, 0.25),
        (0.5, 0.5),
        (0.75, 0.75),
        (1.0, 1.0),
        (0.999, 0.999),
        (-0.5, 0.0),  // out-of-range below 0 must clamp to 0, not just fall through
        (1.5, 1.0),   // out-of-range above 1 must clamp to 1
    ];

    let mut reproduced = false;
    for (v, expected) in cases {
        let linearized = TransferFunction::Linear.linearize(v);
        let gammaed = TransferFunction::Linear.gamma(v);
        println!(
            "input={v} linearize()={linearized} gamma()={gammaed} expected={expected}"
        );
        if (linearized - expected).abs() > 1e-6 {
            reproduced = true;
        }
        if (gammaed - expected).abs() > 1e-6 {
            reproduced = true;
        }
    }

    if reproduced {
        eprintln!("REPRODUCED - TransferFunction::Linear does not behave as identity on [0,1]");
        std::process::exit(1);
    } else {
        println!("did not reproduce - Linear transfer function behaved as identity");
    }
}
