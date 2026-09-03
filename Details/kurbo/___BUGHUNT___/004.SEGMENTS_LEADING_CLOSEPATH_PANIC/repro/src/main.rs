// Standalone reproduction for: kurbo::segments()/BezPath's Shape methods panic when the very
// first PathEl the iterator produces is PathEl::ClosePath.
//
// Uses only kurbo's public API:
//   - the free function `kurbo::segments()` (the exact minimal repro from the candidate report)
//   - `BezPath::from_vec` + the `Shape::area()` trait method (a realistic call path a caller of
//     kurbo would actually use; `from_vec`'s invariant check is only a `debug_assert!`, so this
//     path panics identically in a release build, and is unguarded at all through the free
//     `segments()` function in every build).
//
// Exit code convention: 0 = did not reproduce, non-zero = reproduced (a panic was caught).

use kurbo::{BezPath, PathEl, Shape};

fn main() {
    let mut reproduced = false;

    // 1. The free function, exactly as described in the candidate report.
    let result = std::panic::catch_unwind(|| {
        kurbo::segments(vec![PathEl::ClosePath].into_iter()).collect::<Vec<_>>()
    });
    match result {
        Ok(segs) => {
            println!("segments(): did not reproduce - got {segs:?}");
        }
        Err(payload) => {
            reproduced = true;
            eprintln!("segments(): REPRODUCED - panicked: {:?}", panic_message(&payload));
        }
    }

    // 2. A realistic higher-level entry point: BezPath::area() (a Shape method), on a BezPath
    // built via the public from_vec constructor whose first element is ClosePath. In a release
    // build (debug-assertions off, the common case for a published crate's downstream users)
    // from_vec's own debug_assert! is compiled out, so this path is reachable without ever
    // hitting from_vec's own panic message first.
    let result = std::panic::catch_unwind(|| {
        let path = BezPath::from_vec(vec![PathEl::ClosePath]);
        path.area()
    });
    match result {
        Ok(area) => {
            println!("BezPath::area(): did not reproduce - got {area}");
        }
        Err(payload) => {
            reproduced = true;
            eprintln!(
                "BezPath::area(): REPRODUCED - panicked: {:?}",
                panic_message(&payload)
            );
        }
    }

    if reproduced {
        std::process::exit(1);
    }
}

fn panic_message(payload: &Box<dyn std::any::Any + Send>) -> String {
    if let Some(s) = payload.downcast_ref::<&str>() {
        s.to_string()
    } else if let Some(s) = payload.downcast_ref::<String>() {
        s.clone()
    } else {
        "<non-string panic payload>".to_string()
    }
}
