//! Real end-to-end repro for candidate 031: `rustix::event::fd_set_insert`/`fd_set_remove`
//! (both fully safe, non-`unsafe` public functions) panic with an out-of-bounds slice index
//! when handed a negative `RawFd` - e.g. a `-1` error sentinel a caller forwarded without
//! checking it first, a very common C/POSIX convention for "no valid fd here".
//!
//! This calls rustix's real, unmodified, public API directly - no second participant, no
//! hand-built internal state: a negative `RawFd` is itself the genuine external input these
//! functions are documented to accept (the parameter type is plain `RawFd` = `i32`, which is
//! trivially negative-representable, and nothing in the public docs says otherwise).

use rustix::event::{fd_set_insert, fd_set_remove, FdSetElement, FdSetIter};
use std::panic::{self, UnwindSafe};

fn try_call(name: &str, f: impl FnOnce() + UnwindSafe) -> bool {
    match panic::catch_unwind(f) {
        Ok(()) => {
            println!("{name}: completed, no panic");
            true
        }
        Err(e) => {
            let msg = e
                .downcast_ref::<String>()
                .map(|s| s.as_str())
                .or_else(|| e.downcast_ref::<&str>().copied())
                .unwrap_or("<unknown panic payload>");
            println!("{name}: PANICKED: {msg}");
            false
        }
    }
}

fn main() {
    let mut any_panicked = false;

    // The realistic trigger: a caller that looked up a file descriptor through some mechanism
    // using the common C convention of "-1 means not found/invalid" and forwarded it straight
    // into fd_set_insert without validating it first - no unsafe code anywhere in this program.
    let looked_up_fd: rustix::fd::RawFd = -1;

    for fd in [looked_up_fd, -2, i32::MIN] {
        let mut fds = [FdSetElement::default(); 4];
        any_panicked |= !try_call(&format!("fd_set_insert(fd={fd})"), move || {
            fd_set_insert(&mut fds, fd)
        });

        let mut fds = [FdSetElement::default(); 4];
        any_panicked |= !try_call(&format!("fd_set_remove(fd={fd})"), move || {
            fd_set_remove(&mut fds, fd)
        });
    }

    // Also check a positive fd that's simply too large for the caller's own (correctly sized
    // for a smaller nfds) storage - same OOB-index shape, no sign conversion even needed.
    let mut tiny = [FdSetElement::default(); 1];
    any_panicked |= !try_call("fd_set_insert(fd=100000, tiny array)", move || {
        fd_set_insert(&mut tiny, 100_000)
    });

    // Sanity check: a normal, legitimately in-range fd must still behave correctly either way.
    let mut fds = [FdSetElement::default(); 4];
    fd_set_insert(&mut fds, 5);
    let present: Vec<_> = FdSetIter::new(&fds).collect();
    println!("sanity: fd=5 insert -> set contains {present:?}");
    assert_eq!(present, vec![5], "legitimate in-range insert must still work");
    fd_set_remove(&mut fds, 5);
    let present: Vec<_> = FdSetIter::new(&fds).collect();
    println!("sanity: fd=5 remove -> set contains {present:?}");
    assert_eq!(present, Vec::<i32>::new(), "legitimate in-range remove must still work");

    if any_panicked {
        println!("RESULT: BUG REPRODUCED (a safe public API panicked on a negative/out-of-range RawFd)");
        std::process::exit(1);
    } else {
        println!("RESULT: no panic from any negative/out-of-range RawFd (fix is in effect)");
    }
}
