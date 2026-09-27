// Standalone reproduction for candidate 033
// (EXPAND_JOIN_ZERO_EXPAND_SILENT_BEVEL_FALLBACK).
//
// `ExpandCtx::do_join` (kurbo/src/expand.rs) gates entry into the Miter/Round join code with:
//
//     let cross = self.last_tan.cross(tan);
//     if cross * self.expand.xx < 0.0 { ... match self.join { Miter => ..., Round => ..., } }
//
// `expand.xx` is used purely as a sign gate here (see the doc on `expand_path`: "expand.xx" and
// "expand.yy" are expected to share the same sign). But when `expand.xx == 0.0` exactly, the
// product `cross * self.expand.xx` is always `0.0`, which is never `< 0.0` - so the guard is
// always false, and the function silently falls through to the Bevel-join code no matter what
// `Join` the caller actually asked for. This reproduction constructs a path with a sharp corner
// and an anisotropic expansion with `expand.xx == 0.0` and `expand.yy` nonzero, then shows that
// `Join::Miter` and `Join::Round` produce byte-for-byte the same output as `Join::Bevel` - which
// they should not, and (per a sanity check against a "same sign, tiny nonzero xx" input) do not
// once `expand.xx` is any nonzero value with the same sign.
//
// Exit code convention: 0 = did not reproduce (bug absent), non-zero = reproduced (bug present).

use kurbo::{BezPath, Diagonal2, Join, Point, expand_path_signed};

fn corner_path() -> BezPath {
    let mut path = BezPath::new();
    path.move_to(Point::new(0.0, 0.0));
    path.line_to(Point::new(10.0, 0.0));
    path.line_to(Point::new(10.0, 10.0));
    path.close_path();
    path
}

fn main() {
    let path = corner_path();
    let tol = 0.1;
    let miter_limit = 4.0;

    // expand.xx == 0.0 exactly, expand.yy negative (this specific corner/sign combination is the
    // one that actually needs a real Miter/Round join, as the sanity check below confirms).
    let expand_zero_xx = Diagonal2::new(0.0, -2.0);

    let bevel = expand_path_signed(path.clone(), expand_zero_xx, Join::Bevel, miter_limit, tol);
    let miter = expand_path_signed(path.clone(), expand_zero_xx, Join::Miter, miter_limit, tol);
    let round = expand_path_signed(path.clone(), expand_zero_xx, Join::Round, miter_limit, tol);

    let bevel_els: Vec<_> = bevel.elements().to_vec();
    let miter_els: Vec<_> = miter.elements().to_vec();
    let round_els: Vec<_> = round.elements().to_vec();

    println!("expand.xx == 0.0, expand.yy == -2.0:");
    println!("  Bevel: {bevel_els:?}");
    println!("  Miter: {miter_els:?}");
    println!("  Round: {round_els:?}");

    // Sanity check: with a tiny nonzero expand.xx of the SAME sign convention, Miter/Round are
    // expected to genuinely differ from Bevel for this exact corner (proves this isn't just a
    // degenerate corner where Bevel is the mathematically correct answer regardless of join type).
    let expand_tiny_xx = Diagonal2::new(-1e-6, -2.0);
    let bevel_tiny = expand_path_signed(path.clone(), expand_tiny_xx, Join::Bevel, miter_limit, tol);
    let miter_tiny = expand_path_signed(path.clone(), expand_tiny_xx, Join::Miter, miter_limit, tol);
    let bevel_tiny_els: Vec<_> = bevel_tiny.elements().to_vec();
    let miter_tiny_els: Vec<_> = miter_tiny.elements().to_vec();
    let sanity_ok = miter_tiny_els != bevel_tiny_els;
    println!(
        "sanity check (expand.xx = -1e-6, same corner): Miter != Bevel -> {sanity_ok}"
    );
    if !sanity_ok {
        eprintln!(
            "INCONCLUSIVE: even a tiny nonzero same-sign expand.xx doesn't make Miter diverge \
             from Bevel for this corner; the repro's premise no longer holds"
        );
        std::process::exit(2);
    }

    let miter_matches_bevel = miter_els == bevel_els;
    let round_matches_bevel = round_els == bevel_els;

    if miter_matches_bevel && round_matches_bevel {
        eprintln!(
            "REPRODUCED: with expand.xx == 0.0 exactly, Join::Miter and Join::Round both \
             silently produced identical output to Join::Bevel, even though the sanity check \
             above proves this corner needs a real Miter/Round join under the same sign \
             convention."
        );
        std::process::exit(1);
    }

    println!(
        "did not reproduce - Join::Miter/Join::Round diverged from Join::Bevel as expected \
         even with expand.xx == 0.0"
    );
}
