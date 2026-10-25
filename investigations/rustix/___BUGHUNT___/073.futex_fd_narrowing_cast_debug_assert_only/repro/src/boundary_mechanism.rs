// NOT a reproduction of the real bug (no real kernel FUTEX_FD syscall return can ever reach this
// value - see RESULT.md for why: FUTEX_FD is unimplemented/ENOSYS on every Linux kernel since
// 4.5 (2016), and even on a hypothetical kernel where it still worked, a real fd number is
// structurally capped below i32::MAX by the kernel's own fs.nr_open ceiling). This exists purely
// to document the root-cause *mechanism* in isolation: it hand-constructs the usize value a
// genuine FUTEX_FD syscall return would need to produce for the cast in `futex::fd()`
// (src/thread/futex.rs:102-110) to misbehave, replicating that function's exact two lines, to
// show the debug-build panic / release-build silent-wrong-fd split described by the candidate.
// Per CONFIRMATION.md section 11, this alone does not and cannot justify a `Confirmed` verdict.

use std::os::fd::RawFd;

fn main() {
    // No real kernel-returned fd can ever be this large (see RESULT.md); hand-fabricated here
    // purely to exercise the cast+debug_assert mechanism in isolation.
    let val: usize = 0xFFFF_FFFE; // would decode as RawFd(-2) if cast
    let fd = val as RawFd;
    println!("hypothetical FUTEX_FD return val = {val} (usize) as RawFd = {fd} (negative: {})", fd < 0);

    // This mirrors futex.rs's own debug_assert_eq! exactly - panics here in a debug build,
    // compiled out entirely in release (matching the candidate's description).
    debug_assert_eq!(fd as usize, val, "return value should be a valid fd");

    println!(
        "reached past debug_assert (only possible in a release build, where debug_assert! is \
         compiled out): fd={fd} would be handed to OwnedFd::from_raw_fd(fd)"
    );
}
