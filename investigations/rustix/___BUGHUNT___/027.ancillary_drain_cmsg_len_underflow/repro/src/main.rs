//! Standalone repro for candidate 027: `AncillaryDrain::advance`'s
//! `**length -= msg_len` (src/net/send_recv/msg.rs) underflows when a
//! `cmsghdr`'s `cmsg_len` is larger than the amount of control-buffer space
//! actually remaining.
//!
//! Two real, independent participants are used for the parts that genuinely
//! are two participants: a sender thread and a receiver (main thread),
//! connected by a real `AF_UNIX` `SOCK_STREAM` socketpair, with the sender
//! doing a real `sendmsg` carrying `NUM_FDS` real, live file descriptors as
//! a real `SCM_RIGHTS` ancillary message, and the receiver doing a real
//! `recvmsg` through rustix's real public API
//! (`RecvAncillaryBuffer`/`AncillaryDrain`) into a buffer deliberately too
//! small to hold them all.
//!
//! The one part that is NOT live here: the actual bug (GitHub issue #1683)
//! is macOS-specific kernel behavior - on macOS, this exact `recvmsg` call
//! genuinely truncates the control data to fit but leaves the `cmsghdr`'s
//! own `cmsg_len` field set to the *untruncated* length, i.e. the real
//! external input that triggers the bug is produced by the macOS kernel
//! itself. No macOS hardware/VM is available in this (Linux x86_64)
//! environment to make that real syscall actually misbehave that way - on
//! this real Linux kernel the same `recvmsg` call correctly keeps `cmsg_len`
//! consistent with the truncated data (confirmed separately in this
//! project's own ___BUGHUNT___/017.recvmsg_ctrunc_ancillary_not_reproducible
//! candidate), so the bug is simply not reachable end-to-end on Linux.
//!
//! Per CONFIRMATION.md section 11's narrow, explicitly-justified exception
//! for a finding that needs hardware that isn't available: after the real
//! `recvmsg` call returns (populating the buffer correctly, per real Linux
//! behavior), this repro directly overwrites just the one `cmsg_len` field,
//! in place, in our own backing array, to the exact value PR #1684's author
//! reports real macOS 15.6 leaves there (the untruncated length) - standing
//! in for the one real participant (the macOS kernel) this environment
//! cannot actually invoke. Everything else - the socket, the sender, the
//! real fds, the real `sendmsg`/`recvmsg` calls, and the real
//! `RecvAncillaryBuffer`/`AncillaryDrain` code under test - is genuine.
//!
//! Exit code convention: 0 = did not reproduce (fixed tree). A build with
//! the bug present will either panic (debug build: "attempt to subtract
//! with overflow", then a second panic from `Drop`'s own re-drain, which
//! aborts the whole process - this cannot be caught with
//! `std::panic::catch_unwind`, by design, since it's a real `SIGABRT`) or,
//! in a release build, silently read past the end of the buffer and keep
//! going (observed separately; see output.txt) - either way, a nonzero exit
//! code / abort means reproduced.

use rustix::fd::AsFd;
use rustix::io::{IoSlice, IoSliceMut};
use rustix::net::{
    recvmsg, sendmsg, socket, socketpair, AddressFamily, RecvAncillaryBuffer,
    RecvAncillaryMessage, RecvFlags, SendAncillaryBuffer, SendAncillaryMessage, SendFlags,
    SocketFlags, SocketType,
};
use std::mem::MaybeUninit;
use std::thread;

/// Number of file descriptors the sender genuinely sends in one `SCM_RIGHTS`
/// message.
const NUM_FDS: usize = 16;
/// Number of file descriptors' worth of space the receiver's control buffer
/// actually has room for - deliberately smaller than `NUM_FDS` so the real
/// kernel is forced to truncate the control data.
const NUM_SLOTS: usize = 5;

fn main() {
    let (send_sock, recv_sock) = socketpair(
        AddressFamily::UNIX,
        SocketType::STREAM,
        SocketFlags::empty(),
        None,
    )
    .expect("socketpair");

    // Real, independent sender participant, on its own thread, with its own
    // real file descriptors to pass.
    let sender = thread::spawn(move || {
        let fds: Vec<_> = (0..NUM_FDS)
            .map(|_| socket(AddressFamily::UNIX, SocketType::STREAM, None).expect("socket"))
            .collect();
        let borrowed: Vec<_> = fds.iter().map(AsFd::as_fd).collect();

        let mut send_space = [MaybeUninit::uninit(); rustix::cmsg_space!(ScmRights(NUM_FDS))];
        let mut send_cmsg = SendAncillaryBuffer::new(send_space.as_mut_slice());
        assert!(send_cmsg.push(SendAncillaryMessage::ScmRights(&borrowed)));

        sendmsg(
            &send_sock,
            &[IoSlice::new(b"hello")],
            &mut send_cmsg,
            SendFlags::empty(),
        )
        .expect("sendmsg");
        // Keep `fds` (and `send_sock`) alive until the real sendmsg above
        // has handed them to the kernel.
        drop(fds);
    });

    let mut recv_space = [MaybeUninit::uninit(); rustix::cmsg_space!(ScmRights(NUM_SLOTS))];
    // Raw pointer into our own backing array, taken before it's mutably
    // borrowed by `RecvAncillaryBuffer` - used below, strictly after the
    // real `recvmsg` call has returned, to directly simulate the one
    // OS-specific field macOS's real kernel would have left behind (see the
    // module doc comment above for why this one field is simulated instead
    // of genuinely produced).
    let raw_ptr = recv_space.as_mut_ptr();

    let mut recv_cmsg = RecvAncillaryBuffer::new(recv_space.as_mut_slice());

    let mut buf = [0_u8; 5];
    let result = recvmsg(
        &recv_sock,
        &mut [IoSliceMut::new(&mut buf)],
        &mut recv_cmsg,
        RecvFlags::empty(),
    )
    .expect("recvmsg");
    println!(
        "real recvmsg(): bytes={} flags={:?}",
        result.bytes, result.flags
    );
    assert_eq!(result.bytes, 5);
    assert_eq!(&buf, b"hello");

    sender.join().expect("sender thread panicked");

    // Simulate macOS's documented quirk: overwrite the real, kernel-written
    // `cmsg_len` of the (single, truncated) cmsghdr to the untruncated
    // value it would have had, had none of the SCM_RIGHTS payload been cut
    // off. `cmsg_len` is the first field of `cmsghdr` on every platform this
    // crate supports (Linux, the BSDs, macOS).
    let untruncated_cmsg_len: usize =
        unsafe { libc::CMSG_LEN((NUM_FDS * std::mem::size_of::<i32>()) as _) } as usize;
    unsafe {
        std::ptr::write_unaligned(raw_ptr.cast::<usize>(), untruncated_cmsg_len);
    }
    println!("simulated macOS-style stale cmsg_len = {untruncated_cmsg_len}");

    // This call exercises the real `RecvAncillaryBuffer::drain()` /
    // `AncillaryDrain::advance` / `cvt_msg` path on the buffer above. On the
    // unpatched tree this underflows `length` and panics; unwinding then
    // runs `RecvAncillaryBuffer`'s `Drop`, which drains again and panics a
    // second time while already panicking, which aborts the process
    // (`SIGABRT`) before we ever get back here.
    let mut received = 0usize;
    for msg in recv_cmsg.drain() {
        match msg {
            RecvAncillaryMessage::ScmRights(rights) => received += rights.count(),
            _ => panic!("unexpected ancillary message"),
        }
    }
    println!("first drain() completed, received {received} fds (not yet conclusive - see below)");

    // Force the second drain explicitly (rather than relying on implicit
    // end-of-scope `Drop`) so a release build's success/failure is decided
    // right here, not interleaved with later code. On the unpatched tree,
    // a release build reaches this point (no debug-assertion panic took
    // place above) but panics *here*, on the real `Drop` impl's own
    // re-`drain()` of the now-corrupted `length` bookkeeping.
    drop(recv_cmsg);

    println!("did not reproduce - both drains completed without panicking, received {received} fds");
}
