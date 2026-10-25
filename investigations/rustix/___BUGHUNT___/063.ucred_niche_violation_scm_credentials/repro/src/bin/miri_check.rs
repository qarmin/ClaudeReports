//! Miri-only variant: no real socket syscalls (Miri can't execute rustix's
//! real `recvmsg`/`sendmsg` syscalls - see ../../../README.md). Uses the
//! exact genuine byte template captured from a real `sendmsg`/`recvmsg`
//! SCM_CREDENTIALS round trip (see `src/bin/end_to_end.rs`'s step 1 output),
//! with the pid field patched to 0 the same way, then feeds it through the
//! real public `AncillaryDrain::parse` entry point - this is where the
//! actual Undefined Behavior this candidate is about is caught.
//!
//! Run with:
//!   RUSTFLAGS="--cfg rustix_no_linux_raw" \
//!   MIRIFLAGS="-Zmiri-disable-stacked-borrows" \
//!   cargo +nightly miri run --bin miri_check
//!
//! `--cfg rustix_no_linux_raw` forces rustix's `libc` backend, since the
//! `linux_raw` backend's inline `asm!` syscalls can't run under Miri at all.
//!
//! `-Zmiri-disable-stacked-borrows` is required to reach the finding this
//! candidate is about at all: the surrounding `AncillaryDrain::cvt_msg`
//! parsing code (specifically the `CMSG_DATA`/`slice::from_raw_parts_mut`
//! reborrow dance just above the line under test) independently trips an
//! unrelated Stacked Borrows violation ("retag ... Unique permission ...
//! tag does not exist in the borrow stack") on *every* SCM_CREDENTIALS
//! message, valid or not - this is a distinct, pre-existing aliasing-model
//! finding in the same function, out of scope for this candidate (noted as
//! a new suspect in RESULT.md), not the niche-violation bug under test
//! here. Disabling Stacked Borrows checking isolates the one specific,
//! separate class of UB (constructing an invalid value for a
//! niche-optimized type) this candidate investigates; it does not disable
//! Miri's validity-invariant checking, which is what actually catches the
//! bug below.

use rustix::net::{AncillaryDrain, RecvAncillaryMessage};

// `cmsghdr` requires 8-byte alignment; a plain `[u8; N]` on the stack isn't
// guaranteed to have it, which trips an unrelated alignment UB before ever
// reaching the bug under test. Force it explicitly, matching what
// `RecvAncillaryBuffer`'s own `align_for_cmsghdr` does for a real buffer.
#[repr(align(8))]
struct AlignedBuf([u8; 40]);

fn main() {
    // Genuine bytes captured from a real `recvmsg()` SCM_CREDENTIALS
    // delivery (rustix 287214b889865d8e1406a0ee71cc409b6f6191c8, x86_64
    // linux, the sandbox this candidate was investigated in):
    // cmsg_len=0x1c cmsg_level=1(SOL_SOCKET) cmsg_type=2(SCM_CREDENTIALS)
    // payload: pid=338228 uid=1000 gid=1000, then alignment padding.
    let mut buf = AlignedBuf([
        0x1c, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // cmsg_len (size_t)
        0x01, 0x00, 0x00, 0x00, // cmsg_level = SOL_SOCKET
        0x02, 0x00, 0x00, 0x00, // cmsg_type = SCM_CREDENTIALS
        0x34, 0x29, 0x05, 0x00, // pid = 338228 (real genuine value)
        0xe8, 0x03, 0x00, 0x00, // uid = 1000
        0xe8, 0x03, 0x00, 0x00, // gid = 1000
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // padding
    ]);

    // Patch only the pid field to 0 - the one field that requires a real
    // cross-PID-namespace kernel delivery to produce for real, which this
    // sandbox cannot construct (`unshare(CLONE_NEWUSER/CLONE_NEWPID)` both
    // return EPERM here). See end_to_end.rs's module doc for the full
    // justification.
    buf.0[16..20].copy_from_slice(&0i32.to_ne_bytes());

    let mut drain = unsafe { AncillaryDrain::parse(&mut buf.0) };
    match drain.next() {
        Some(RecvAncillaryMessage::ScmCredentials(bad_ucred)) => {
            // Pre-fix: this line is reached, but under Miri the
            // `read_unaligned` that produced `bad_ucred` already aborted
            // with "constructing invalid value ... encountered 0, but
            // expected something greater or equal to 1" before getting
            // here - this print is only reachable at all outside Miri.
            eprintln!("BUG: constructed UCred with invalid Pid: {bad_ucred:?}");
        }
        None => {
            // Post-fix: the pid=0 payload is recognized as unrepresentable
            // and the ancillary message is skipped instead of constructing
            // an invalid `Pid`.
            eprintln!("FIXED: pid=0 credentials message was safely skipped");
        }
        Some(_) => panic!("unexpected non-ScmCredentials message"),
    };
}
