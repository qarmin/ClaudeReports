// Repro for candidate 022: `PathSeg::min_dist(&self, other: PathSeg, accuracy: f64)`
// (kurbo/src/bezpath.rs) is a fully public entry point that passes its caller-supplied `accuracy`
// straight through, unclamped, as `mindist::min_dist_param`'s `epsilon` - the *only* termination
// check for that function's recursive quadrant-subdivision search
// (`(umax - umin).abs() < epsilon || (vmax - vmin).abs() < epsilon`, mindist.rs). For
// `accuracy == 0.0` this check can never become true (`x.abs() < 0.0` is false for every finite
// `x`, including `0.0` itself), so recursion has no way to terminate on its own; no depth counter
// exists anywhere in the function, so it runs until the call stack is exhausted.
//
// This uses kurbo's real, unmodified public API (`PathSeg::min_dist`) with two ordinary `Line`
// segments as input - exactly what any real caller would construct and pass. No second
// participant is being faked: `accuracy` here is a single f64 supplied directly by the immediate
// caller of a public API, not something produced by an upstream peer/protocol - the single-process
// repro shape is the correct/complete one for this finding (CONFIRMATION.md section 11's "single
// external artifact, no other side" case applies).
//
// A stack overflow aborts the whole process (SIGSEGV/"has overflowed its stack") rather than
// unwinding as a catchable panic, so this repro can't `catch_unwind` it in-process - the crash
// itself (non-zero/aborted exit, "has overflowed its stack" on stderr) *is* the observable
// reproduction. If the call instead returns normally, the bug is absent (fixed).

use kurbo::{Line, PathSeg};

fn main() {
    let seg1 = PathSeg::Line(Line::new((0.0, 0.0), (10.0, 0.0)));
    let seg2 = PathSeg::Line(Line::new((0.0, 5.0), (10.0, 5.0)));

    println!("calling PathSeg::min_dist(other, accuracy = 0.0) on two plain Line segments...");
    println!("(a stack overflow here - process aborts, no panic to catch - IS the reproduction)");

    let result = seg1.min_dist(seg2, 0.0);

    // Only reached if the bug is fixed (recursion terminated on its own).
    println!(
        "did not reproduce - min_dist returned normally: distance={}, t1={}, t2={}",
        result.distance, result.t1, result.t2
    );
}
