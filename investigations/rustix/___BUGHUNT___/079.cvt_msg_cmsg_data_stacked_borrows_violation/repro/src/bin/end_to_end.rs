//! Real end-to-end demonstration: a genuine `AF_UNIX` `socketpair`, a real
//! `sendmsg`/`recvmsg` round trip carrying a real `SCM_RIGHTS` ancillary
//! message through rustix's own public API, then parsing the result via the
//! real public `RecvAncillaryBuffer::drain()` entry point - exactly the
//! trigger path `AncillaryDrain::cvt_msg` (`src/net/send_recv/msg.rs`) runs
//! on every received ancillary message, valid or not. No adversarial input
//! and no hand-constructed bytes are needed here at all: this bug is a pure
//! Stacked Borrows violation in rustix's own pointer handling, not a
//! reaction to any unusual peer behavior, so an entirely ordinary fd-passing
//! round trip between two fully cooperative ends already triggers it.
//!
//! This binary does NOT crash in a plain (non-Miri) build - the bug is real
//! Undefined Behavior per the Rust abstract machine regardless, but doesn't
//! manifest as an externally visible crash without an instrumented tool
//! (CONFIRMATION.md section 2). See `src/bin/miri_check.rs` for the binary
//! that actually demonstrates the UB.
//!
//! Why this binary exists at all, given that: Miri cannot execute rustix's
//! real `sendmsg`/`recvmsg` syscalls (confirmed directly - running this same
//! real round trip under Miri aborts on the *send* side, on an unrelated
//! Stacked Borrows violation in `push_ancillary`, before `sendmsg` is even
//! called - see RESULT.md). So this binary establishes that the byte layout
//! used in `miri_check.rs` is genuine - captured verbatim from this real
//! `recvmsg()` call, not guessed or hand-authored - by printing it at the
//! end.

use std::mem::MaybeUninit;
use std::os::fd::AsFd;

use rustix::io::{IoSlice, IoSliceMut};
use rustix::net::{
    recvmsg, sendmsg, socketpair, AddressFamily, RecvAncillaryBuffer, RecvAncillaryMessage,
    RecvFlags, SendAncillaryBuffer, SendAncillaryMessage, SendFlags, SocketFlags, SocketType,
};

fn main() {
    let (send_sock, recv_sock) = socketpair(
        AddressFamily::UNIX,
        SocketType::STREAM,
        SocketFlags::CLOEXEC,
        None,
    )
    .unwrap();

    // Send the sender socket's own fd as a real SCM_RIGHTS ancillary
    // message - any ordinary fd-passing use of this API triggers the bug,
    // nothing adversarial needed.
    let fds = [send_sock.as_fd()];
    let msg = SendAncillaryMessage::ScmRights(&fds);
    let mut space = [MaybeUninit::uninit(); rustix::cmsg_space!(ScmRights(1))];
    let mut send_cmsg_buffer = SendAncillaryBuffer::new(space.as_mut_slice());
    assert!(send_cmsg_buffer.push(msg));

    sendmsg(
        &send_sock,
        &[IoSlice::new(b"x")],
        &mut send_cmsg_buffer,
        SendFlags::empty(),
    )
    .unwrap();

    let mut cmsg_space = [MaybeUninit::uninit(); rustix::cmsg_space!(ScmRights(1))];
    let mut data_buf = [0u8; 16];

    let base_addr = cmsg_space.as_ptr() as usize;
    let align = std::mem::align_of::<libc::cmsghdr>();

    let mut recv_cmsg_buffer = RecvAncillaryBuffer::new(cmsg_space.as_mut_slice());
    let result = recvmsg(
        &recv_sock,
        &mut [IoSliceMut::new(&mut data_buf)],
        &mut recv_cmsg_buffer,
        RecvFlags::empty(),
    )
    .unwrap();
    println!("recvmsg bytes={}", result.bytes);

    // This is the real trigger path: AncillaryDrain::cvt_msg runs
    // CMSG_DATA(msg) on a shared &cmsghdr reborrow, then builds a *mutable*
    // slice from the resulting pointer via slice::from_raw_parts_mut - a
    // Stacked Borrows violation on every single received ancillary message,
    // valid or not, regardless of its contents.
    match recv_cmsg_buffer.drain().next() {
        Some(RecvAncillaryMessage::ScmRights(mut fds)) => {
            let fd = fds.next().unwrap();
            println!("received fd = {:?}", fd);
        }
        _ => panic!("unexpected ancillary message"),
    }
    println!("done (no crash in a plain, non-Miri build)");

    // Print the exact genuine bytes the kernel wrote, for embedding verbatim
    // into the Miri-only variant (Miri can't execute the real sendmsg/recvmsg
    // syscalls - see ../../README.md).
    let aligned_addr = (base_addr + (align - 1)) & align.wrapping_neg();
    let pad = aligned_addr - base_addr;
    let total_len = rustix::cmsg_space!(ScmRights(1));
    let raw: &[u8] = unsafe { std::slice::from_raw_parts(aligned_addr as *const u8, total_len - pad) };
    println!("genuine captured cmsg bytes = {raw:02x?}");
}
