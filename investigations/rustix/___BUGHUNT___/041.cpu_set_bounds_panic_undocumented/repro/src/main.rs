// Standalone reproduction for candidate 041: `rustix::thread::CpuSet::set`/`unset`/`is_set`
// panic on an out-of-range `field` (>= `CpuSet::MAX_CPU`, 1024 on a 64-bit target), with no
// mention of this in the public doc comments, and with a backend-dependent panic message (a
// generic Rust "index out of bounds" on the `linux_raw` backend vs. a clear custom assertion
// message on the `libc` backend) for what should be identical observable behavior.
//
// Exit code convention: 0 = did not reproduce, non-zero/panic = reproduced.

fn main() {
    let max_cpu = rustix::thread::CpuSet::MAX_CPU;
    println!("CpuSet::MAX_CPU = {max_cpu}");

    let mut set = rustix::thread::CpuSet::new();

    // Boundary-value check: the largest in-range CPU id must still work fine.
    set.set(max_cpu - 1);
    assert!(set.is_set(max_cpu - 1));
    println!("set({}) (in-bounds, MAX_CPU - 1) succeeded as expected", max_cpu - 1);

    // The actual bug: the first out-of-range id (and, per the report, any larger value such as
    // 99999) panics instead of returning a `Result`/documented error - and the panic message
    // differs across backends for what the public API presents as identical behavior.
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let mut set = rustix::thread::CpuSet::new();
        set.set(99999);
    }));

    match result {
        Ok(()) => {
            println!("did not reproduce - CpuSet::set(99999) completed without panicking");
            std::process::exit(1);
        }
        Err(panic_payload) => {
            let msg = panic_payload
                .downcast_ref::<String>()
                .cloned()
                .or_else(|| panic_payload.downcast_ref::<&str>().map(|s| s.to_string()))
                .unwrap_or_else(|| "<non-string panic payload>".to_string());
            eprintln!("panic message: {msg}");

            // Both backends are expected to panic on an out-of-range CPU id (that part is
            // intentional, documented behavior post-fix) - the bug is the *generic, unexplained*
            // raw Rust indexing panic on the `linux_raw` backend, as opposed to a clear, explicit
            // "cpu out of bounds" assertion (what the sibling `libc` backend already did, and
            // what `linux_raw` now does too after the fix).
            if msg.contains("index out of bounds") {
                eprintln!(
                    "REPRODUCED - generic, undocumented 'index out of bounds' panic (pre-fix \
                     linux_raw behavior)"
                );
                std::process::exit(1);
            } else if msg.contains("cpu out of bounds") {
                println!(
                    "did not reproduce - clear, consistent 'cpu out of bounds' panic message \
                     (post-fix behavior, matches the libc backend)"
                );
            } else {
                eprintln!("REPRODUCED - unexpected panic message shape: {msg}");
                std::process::exit(1);
            }
        }
    }
}
