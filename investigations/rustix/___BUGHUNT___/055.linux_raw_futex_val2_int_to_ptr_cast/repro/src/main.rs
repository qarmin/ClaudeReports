use rustix::thread::futex;
use std::sync::atomic::AtomicU32;

fn main() {
    let a = AtomicU32::new(0);
    let b = AtomicU32::new(0);
    // val2 = 1 forces the int-to-ptr cast at
    // src/backend/linux_raw/thread/syscalls.rs:222 (the linux_raw sibling of
    // candidate 004's libc-backend cast) to build a non-null,
    // never-dereferenced "pointer" purely to carry an integer through the
    // `timeout` argument slot FUTEX_REQUEUE reuses for `val2`.
    let _ = futex::requeue(&a, futex::Flags::empty(), 0, 1, &b);
    println!("done");
}
