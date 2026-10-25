use rustix::thread::futex;
use std::sync::atomic::AtomicU32;

fn main() {
    let a = AtomicU32::new(0);
    let b = AtomicU32::new(0);
    // val2 = 1 forces the int-to-ptr cast at
    // src/backend/libc/thread/syscalls.rs:478 to build a non-null,
    // non-dereferenced "pointer" purely to carry an integer through the
    // syscall's `timeout` argument slot.
    let _ = futex::requeue(&a, futex::Flags::empty(), 0, 1, &b);
}
