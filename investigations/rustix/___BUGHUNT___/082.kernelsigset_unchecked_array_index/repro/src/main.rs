// Standalone reproduction for candidate 082: `rustix::io_uring::KernelSigSet::insert`/`remove`/
// `contains` index a fixed-size `kernel_sigset_t` array with no bounds check at all
// (`self.0.sig[raw / sigs_per_elt]`), where `raw` is derived directly from
// `Signal::as_raw().wrapping_sub(1)`. The only way to obtain a `Signal` whose raw value is
// outside the array's valid range (1..=64 on every architecture except MIPS, which uses 1..=128)
// is through `Signal::from_raw_unchecked`/`from_raw_nonzero_unchecked` - both `unsafe` functions
// whose own doc comments already require "a valid and non-zero signal number" as a precondition.
// So this finding needs an already-unsafe precondition violation to trigger at all; it never
// fires from 100% safe code. See RESULT.md for the full reachability discussion.
//
// Exit code convention: 0 = did not reproduce, non-zero/panic = reproduced.

use rustix::io_uring::{KernelSigSet, Signal};

fn try_insert(raw: i32) -> Result<(), String> {
    std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let mut set = KernelSigSet::empty();
        // SAFETY: deliberately NOT upheld - `raw` is intentionally out of range here to probe
        // whether `KernelSigSet` has any defense-in-depth bounds check of its own.
        let sig = unsafe { Signal::from_raw_unchecked(raw) };
        set.insert(sig);
    }))
    .map_err(|payload| {
        payload
            .downcast_ref::<String>()
            .cloned()
            .or_else(|| payload.downcast_ref::<&str>().map(|s| s.to_string()))
            .unwrap_or_else(|| "<non-string panic payload>".to_string())
    })
}

fn main() {
    // Boundary-value check: the largest in-range raw signal number (64 on every arch but MIPS,
    // which this repro doesn't target) must still work fine.
    let in_range = try_insert(64);
    assert!(in_range.is_ok(), "in-range insert(64) unexpectedly panicked: {in_range:?}");
    println!("insert(64) (in-bounds, last valid signal number) succeeded as expected");

    // The actual bug: an out-of-range raw signal number panics via a raw, undocumented Rust
    // array-indexing bounds check instead of a clear, explicit assertion (or any graceful
    // handling) - exercised here with both a too-large value and a negative one, matching the
    // two distinct wrap-around shapes `wrapping_sub(1) as usize` can produce.
    let mut reproduced = false;
    for raw in [100_000i32, -5] {
        match try_insert(raw) {
            Ok(()) => {
                println!("did not reproduce - insert({raw}) completed without panicking");
                std::process::exit(1);
            }
            Err(msg) => {
                eprintln!("insert({raw}) panic message: {msg}");
                if msg.starts_with("index out of bounds") {
                    eprintln!(
                        "REPRODUCED - generic, undocumented 'index out of bounds' panic \
                         (pre-fix behavior) for raw={raw}"
                    );
                    reproduced = true;
                } else if msg.starts_with("signal number out of range") {
                    println!(
                        "did not reproduce - clear, explicit 'signal number out of range' \
                         assertion (post-fix behavior) for raw={raw}"
                    );
                } else {
                    eprintln!("REPRODUCED - unexpected panic message shape: {msg}");
                    std::process::exit(1);
                }
            }
        }
    }

    // The pre-fix shape additionally miscomputes the array length used in the panic message
    // itself ("the len is 1 but the index is ...") in a way that gives no hint the input signal
    // number was ever out of range - captured above via the exact message text, not re-asserted
    // separately here.
    std::process::exit(if reproduced { 1 } else { 0 });
}
