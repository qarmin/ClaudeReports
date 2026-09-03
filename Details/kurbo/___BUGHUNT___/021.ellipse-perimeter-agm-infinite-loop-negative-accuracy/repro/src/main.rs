// Standalone reproduction for candidate 021: `Ellipse::perimeter` (the public `Shape::perimeter`
// impl, kurbo/src/ellipse.rs) hangs forever when called with a non-positive or NaN `accuracy` on
// a non-circular ellipse, because it dispatches to `agm_elliptic_perimeter`'s unguarded `loop`
// whose only exit condition (`term <= accuracy * g`) can never become true once `accuracy <= 0.0`
// or `accuracy` is NaN.
//
// This is a single-process, single-participant repro (a bad-but-in-range f64 argument handed
// directly to a public API by its caller) - there is no second participant/peer whose behavior
// would need to be faked here, so CONFIRMATION.md section 11's stricter bar for two-participant
// findings does not apply.
//
// Exit code convention: 0 = did not reproduce (bug absent), non-zero = reproduced (call hung and
// was killed by the watchdog timeout below). We can't just let it hang forever in a `just test`
// run, so a background watchdog thread aborts the process after a generous timeout and prints
// REPRODUCED - the same outcome an external `timeout` wrapper would observe, just made
// self-contained here.

use std::time::Duration;

use kurbo::{Ellipse, Point, Shape, Vec2};

const WATCHDOG_SECS: u64 = 8;

fn try_one(label: &str, accuracy: f64) {
    println!("--- {label}: perimeter(accuracy={accuracy}) ---");

    let (tx, rx) = std::sync::mpsc::channel();
    std::thread::spawn(move || {
        // Non-circular ellipse (unequal radii) so the trivial-circle fast path in `perimeter`
        // (which never reaches `agm_elliptic_perimeter` at all) is not taken.
        let ellipse = Ellipse::new(Point::new(0.0, 0.0), Vec2::new(2.0, 1.0), 0.0);
        let p = ellipse.perimeter(accuracy);
        let _ = tx.send(p);
    });

    match rx.recv_timeout(Duration::from_secs(WATCHDOG_SECS)) {
        Ok(p) => println!("did not reproduce - perimeter() returned {p} within {WATCHDOG_SECS}s"),
        Err(_) => {
            eprintln!(
                "REPRODUCED - perimeter(accuracy={accuracy}) did not return within {WATCHDOG_SECS}s (infinite loop in agm_elliptic_perimeter)"
            );
            std::process::exit(1);
        }
    }
}

fn main() {
    // A valid, positive accuracy: must complete quickly and is used as a sanity baseline.
    try_one("baseline (valid accuracy)", 1e-6);

    // The actual bug triggers: any accuracy <= 0.0, or NaN.
    let accuracy: f64 = std::env::args()
        .nth(1)
        .map(|s| s.parse().expect("accuracy must parse as f64"))
        .unwrap_or(-1.0);
    try_one("candidate accuracy", accuracy);
}
