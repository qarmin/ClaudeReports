//! Miri-only variant: no real socket syscalls (Miri can't execute rustix's
//! real `sendmsg`/`recvmsg` syscalls - see end_to_end.rs's own attempt,
//! which gets as far as the *send* side before tripping a different,
//! unrelated Stacked Borrows violation in `push_ancillary`, before ever
//! reaching a real `recvmsg`). Uses the exact genuine byte template
//! captured from a real `sendmsg`/`recvmsg` SCM_RIGHTS round trip (see
//! `src/bin/end_to_end.rs`'s own printed output), unmodified - not one byte
//! patched - fed through the real public `AncillaryDrain::parse` entry
//! point, which is where the actual Undefined Behavior this candidate is
//! about is caught.
//!
//! Run with:
//!   RUSTFLAGS="--cfg rustix_no_linux_raw" cargo +nightly miri run --bin miri_check
//!
//! `--cfg rustix_no_linux_raw` forces rustix's `libc` backend, since the
//! `linux_raw` backend's inline `asm!` syscalls can't run under Miri at all.
//!
//! No `-Zmiri-disable-stacked-borrows` here (unlike candidate 063's own
//! miri_check.rs) - Stacked Borrows checking is exactly what this candidate
//! is about.

use rustix::net::{AncillaryDrain, RecvAncillaryMessage};

// `cmsghdr` requires 8-byte alignment; a plain `[u8; N]` on the stack isn't
// guaranteed to have it, which would trip an unrelated alignment UB before
// ever reaching the bug under test. Force it explicitly, matching what
// `RecvAncillaryBuffer`'s own `align_for_cmsghdr` does for a real buffer.
#[repr(align(8))]
struct AlignedBuf([u8; 20]);

fn main() {
    // Genuine, UNMODIFIED bytes captured from a real `recvmsg()` SCM_RIGHTS
    // delivery (rustix 287214b889865d8e1406a0ee71cc409b6f6191c8, x86_64
    // linux, the sandbox this candidate was investigated in):
    // cmsg_len=0x14 cmsg_level=1(SOL_SOCKET) cmsg_type=1(SCM_RIGHTS)
    // payload: one fd = 5. Trailing padding bytes beyond cmsg_len are
    // dropped here (this buffer is sized to exactly cmsg_len, since the
    // bug doesn't depend on any padding beyond the one real message).
    let mut buf = AlignedBuf([
        0x14, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // cmsg_len (size_t) = 20
        0x01, 0x00, 0x00, 0x00, // cmsg_level = SOL_SOCKET
        0x01, 0x00, 0x00, 0x00, // cmsg_type = SCM_RIGHTS
        0x05, 0x00, 0x00, 0x00, // payload: one fd = 5 (real genuine value)
    ]);

    let mut drain = unsafe { AncillaryDrain::parse(&mut buf.0) };
    match drain.next() {
        Some(RecvAncillaryMessage::ScmRights(mut fds)) => {
            // Pre-fix: under Miri, this line is never reached - the
            // CMSG_DATA/slice::from_raw_parts_mut reborrow inside
            // `cvt_msg` already aborted with a Stacked Borrows violation
            // before `cvt_msg` could even return.
            //
            // `fd = 5` isn't a real open file descriptor under Miri (it was
            // never actually opened in this process - this binary never
            // calls a real syscall), so wrap it in `ManuallyDrop` to skip
            // `OwnedFd`'s real `close()` on drop, which would otherwise
            // abort with an unrelated IO-safety violation that has nothing
            // to do with the Stacked Borrows finding under test here.
            let fd = fds.next().map(std::mem::ManuallyDrop::new);
            eprintln!("FIXED: reached past cvt_msg without a Stacked Borrows abort, fd={fd:?}");
        }
        _ => panic!("unexpected ancillary message"),
    };
}
