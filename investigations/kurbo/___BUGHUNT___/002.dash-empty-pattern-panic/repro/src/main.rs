// Standalone reproduction for candidate 002: kurbo::dash() panics on an empty dash pattern
// slice. Calls the real public API (`kurbo::dash`) directly, matching what any caller of this
// documented entry point would do - a single-process, single-participant repro is appropriate
// here since the "input" (the `dashes: &[f64]` slice) is a plain function argument supplied
// directly by the caller, not something produced by a second communicating participant (see
// CONFIRMATION.md section 11).
//
// Exit code convention: 0 = did not reproduce, non-zero = reproduced (panic observed).

use kurbo::PathEl;

fn main() {
    let path = vec![
        PathEl::MoveTo((0.0, 0.0).into()),
        PathEl::LineTo((10.0, 0.0).into()),
    ];

    let result = std::panic::catch_unwind(|| {
        kurbo::dash(path.into_iter(), 0.0, &[]).collect::<Vec<_>>()
    });

    match result {
        Ok(value) => {
            println!("did not reproduce - kurbo::dash returned normally: {value:?}");
        }
        Err(panic_payload) => {
            let msg = panic_payload
                .downcast_ref::<String>()
                .cloned()
                .or_else(|| panic_payload.downcast_ref::<&str>().map(|s| s.to_string()))
                .unwrap_or_else(|| "<non-string panic payload>".to_string());
            eprintln!("REPRODUCED - kurbo::dash panicked: {msg}");
            std::process::exit(1);
        }
    }
}
