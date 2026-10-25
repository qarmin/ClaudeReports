// src/process/prctl.rs:141 (PR_SET_DUMPABLE), reached through the real
// public safe API `rustix::process::set_dumpable_behavior`. Builds a fake
// `*mut c_void` via `config as usize as *mut _` purely to smuggle an enum
// discriminant through prctl(2)'s integer `arg2` slot; the kernel never
// dereferences it.
use rustix::process::DumpableBehavior;

fn main() {
    let _ = rustix::process::set_dumpable_behavior(DumpableBehavior::NotDumpable);
}
