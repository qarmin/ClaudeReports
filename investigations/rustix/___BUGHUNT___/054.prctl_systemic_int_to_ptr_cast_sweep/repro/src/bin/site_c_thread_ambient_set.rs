// src/thread/prctl.rs:757 (PR_CAP_AMBIENT / PR_CAP_AMBIENT_RAISE), reached
// through `rustix::thread::configure_capability_in_ambient_set`. This one
// casts a bare `usize` sub-operation code directly (`sub_operation as *mut
// _`, no intermediate `as usize`) - a slightly different textual shape of
// the exact same integer-to-pointer fabrication bug.
use rustix::thread::CapabilitySet;

fn main() {
    let _ = rustix::thread::configure_capability_in_ambient_set(CapabilitySet::CHOWN, true);
}
