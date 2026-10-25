//! Real end-to-end repro for candidate 102: `rustix::event::fd_set_remove`'s
//! windows/wasi `FD_SET`-array branch swaps the removed slot with the wrong
//! element.
//!
//! Built for `wasm32-wasip1` (same `#[cfg(any(windows, target_os = "wasi"))]`
//! branch the bug lives in) and run under a real wasmtime, so this exercises
//! the actual public `rustix::event::fd_set_insert`/`fd_set_remove`/
//! `FdSetIter` API compiled for a real affected target, not a hand-copied
//! stand-in for the struct layout.

use rustix::event::{fd_set_insert, fd_set_num_elements, fd_set_remove, FdSetElement, FdSetIter};

fn collect(fds: &[FdSetElement]) -> Vec<i32> {
    FdSetIter::new(fds).collect()
}

fn main() {
    let num_elements = fd_set_num_elements(8, 0);
    let mut fds = vec![FdSetElement::default(); num_elements];

    // Build an active set of 3 fds: [5, 10, 15].
    fd_set_insert(&mut fds, 5);
    fd_set_insert(&mut fds, 10);
    fd_set_insert(&mut fds, 15);

    let before = collect(&fds);
    println!("before remove: {:?}", before);
    assert_eq!(before, vec![5, 10, 15], "setup sanity check failed");

    // Remove the non-last fd (10). The correct result is [5, 15] (15 moved
    // into the freed slot). The bug instead reads
    // `set.fd_array.last()` off the full fixed-size `[i32; FD_SETSIZE]`
    // array (always index FD_SETSIZE-1, which is 0 for an otherwise-unused
    // slot) instead of `set.fd_array[fd_count - 1]`, so it inserts a bogus
    // extra fd 0 and silently drops the real fd 15.
    fd_set_remove(&mut fds, 10);

    let after = collect(&fds);
    println!("after remove(10): {:?}", after);

    let expected = vec![5, 15];
    let mut any_failed = false;
    if after == expected {
        println!("RESULT: CORRECT - got {:?} as expected", after);
    } else {
        println!(
            "RESULT: BUG CONFIRMED - expected {:?}, got {:?} (fd 15 dropped and/or bogus fd {:?} introduced)",
            expected,
            after,
            after.iter().find(|&&f| f != 5 && f != 15)
        );
        any_failed = true;
    }

    // Boundary case: removing the *last* active element (pos == fd_count - 1)
    // should still work (this one happened to work even with the bug, since
    // `.last()` on the full array and `fd_array[fd_count-1]` coincide only
    // when fd_count == FD_SETSIZE; verify the fixed code doesn't regress it).
    let num_elements2 = fd_set_num_elements(8, 0);
    let mut fds2 = vec![FdSetElement::default(); num_elements2];
    fd_set_insert(&mut fds2, 1);
    fd_set_insert(&mut fds2, 2);
    fd_set_insert(&mut fds2, 3);
    fd_set_remove(&mut fds2, 3);
    let after2 = collect(&fds2);
    println!("after remove(3) [last-element case]: {:?}", after2);
    if after2 == vec![1, 2] {
        println!("RESULT: CORRECT (last-element case)");
    } else {
        println!(
            "RESULT: BUG CONFIRMED (last-element case) - expected [1, 2], got {:?}",
            after2
        );
        any_failed = true;
    }

    // Draining a set down to empty, one remove at a time.
    let num_elements3 = fd_set_num_elements(8, 0);
    let mut fds3 = vec![FdSetElement::default(); num_elements3];
    fd_set_insert(&mut fds3, 7);
    fd_set_insert(&mut fds3, 8);
    fd_set_remove(&mut fds3, 7);
    fd_set_remove(&mut fds3, 8);
    let after3 = collect(&fds3);
    println!("after draining to empty: {:?}", after3);
    if after3.is_empty() {
        println!("RESULT: CORRECT (drain-to-empty case)");
    } else {
        println!(
            "RESULT: BUG CONFIRMED (drain-to-empty case) - expected [], got {:?}",
            after3
        );
        any_failed = true;
    }

    std::process::exit(if any_failed { 1 } else { 0 });
}
