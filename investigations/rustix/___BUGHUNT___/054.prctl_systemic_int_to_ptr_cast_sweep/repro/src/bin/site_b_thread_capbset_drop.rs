// src/thread/prctl.rs:497 (PR_CAPBSET_DROP), reached through the real
// public safe API `rustix::thread::remove_capability_from_bounding_set`.
// Same `cap as usize as *mut _` cast shape as site_a, in a different file.
use rustix::thread::CapabilitySet;

fn main() {
    let _ = rustix::thread::remove_capability_from_bounding_set(CapabilitySet::CHOWN);
}
