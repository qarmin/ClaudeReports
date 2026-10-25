// Real end-to-end attempt to trigger candidate 073: src/thread/futex.rs:102-110's `fd()` casts
// the raw `FUTEX_FD` syscall return value (`usize`) to `RawFd` (`i32`) with
// `let fd = val as RawFd;`, guarded only by `debug_assert_eq!(fd as usize, val, ...)` (compiled
// out in release builds), before handing it to `OwnedFd::from_raw_fd(fd)`.
//
// This calls rustix's real public `rustix::thread::futex::fd` (the exact suspect function,
// exercising the real kernel syscall - the genuine, unfaked "second participant" whose output
// this cast narrows), not a hand-constructed stand-in for what the kernel would return.
//
// Exit code convention: 0 = did not reproduce, non-zero/panic = reproduced.

use rustix::thread::futex;
use std::sync::atomic::AtomicU32;

fn main() {
    let uaddr = AtomicU32::new(0);

    match futex::fd(&uaddr, futex::Flags::empty(), 0) {
        Ok(owned_fd) => {
            // If this ever succeeds, print what we got - a real kernel-returned fd here would
            // still have to be checked against the i32 boundary to call this reproduced.
            use std::os::fd::AsRawFd;
            println!(
                "futex::fd() returned Ok(fd={}) - unexpected on any kernel >= 4.5 (FUTEX_FD was \
                 removed); would need separate boundary analysis to call this reproduced",
                owned_fd.as_raw_fd()
            );
        }
        Err(e) => {
            println!("futex::fd() returned Err({e:?})");
            println!("raw_os_error = {}", e.raw_os_error());
        }
    }

    println!(
        "kernel release (uname -r / /proc/sys/kernel/osrelease) and fs.nr_open are printed by \
         output.txt's surrounding shell commands, not this binary"
    );
    println!(
        "did not reproduce via the real end-to-end path - see RESULT.md for why FUTEX_FD is \
         unreachable on any supported modern kernel, and why even a hypothetical success return \
         is structurally bounded well below the i32 boundary by the kernel's own fs.nr_open cap"
    );
}
