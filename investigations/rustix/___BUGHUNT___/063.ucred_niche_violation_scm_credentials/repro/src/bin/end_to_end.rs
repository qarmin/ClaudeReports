//! Real end-to-end demonstration: a genuine `AF_UNIX` `socketpair`, a real
//! `sendmsg`/`recvmsg` round trip carrying a real `SCM_CREDENTIALS` message
//! through rustix's own public API, and a capture of the *genuine* kernel-
//! produced `cmsghdr`+`ucred` bytes that round trip produces.
//!
//! This binary does NOT crash either before or after the fix - the bug this
//! candidate is about (constructing an invalid `Pid(0)` via `read_unaligned`)
//! is invisible to a plain (non-Miri) build; it is real Undefined Behavior
//! per the Rust abstract machine regardless, but doesn't manifest as an
//! externally visible crash without an instrumented tool. See
//! `src/bin/miri_check.rs` for the binary that actually demonstrates the UB
//! (that one aborts pre-fix, and runs clean post-fix).
//!
//! Why this binary exists at all, given that: it establishes that the byte
//! layout used in `miri_check.rs` is genuine - captured from this real
//! `recvmsg()` call, not guessed - and it demonstrates (post-fix) that the
//! same patched buffer, when fed through the real public `AncillaryDrain`
//! parsing path, now safely skips the unrepresentable message instead of
//! silently holding an invalid `Pid`.
//!
//! Real kernel behavior that makes `pid == 0` a genuinely reachable value in
//! `SCM_CREDENTIALS` (not just a hostile/invalid payload): Linux fills in the
//! sender's pid via `pid_vnr()`-style translation relative to the
//! *receiver's* own PID namespace at receive time. When the sender's PID
//! cannot be represented there (sender and receiver in unrelated PID
//! namespaces sharing a socket), the kernel delivers `pid == 0`. Reproducing
//! that exact condition for real requires `unshare(CLONE_NEWUSER)`/
//! `CLONE_NEWPID`, both confirmed blocked in the sandbox this candidate was
//! investigated in:
//!   unshare(CLONE_NEWUSER) = -1 errno=1 (Operation not permitted)
//!   unshare(CLONE_NEWPID)  = -1 errno=1 (Operation not permitted)
//! (AppArmor's `apparmor_restrict_unprivileged_userns=1`, and `no_new_privs`
//! blocking the sudo/setuid-helper workaround too.) So this binary patches
//! only the `pid` field of an otherwise-100%-genuine captured buffer to 0 -
//! see RESULT.md for the full justification (CONFIRMATION.md section 11).

use std::mem::MaybeUninit;

use rustix::io::{IoSlice, IoSliceMut};
use rustix::net::{
    recvmsg, sendmsg, socketpair, sockopt, AddressFamily, AncillaryDrain, RecvAncillaryBuffer,
    RecvAncillaryMessage, RecvFlags, SendAncillaryBuffer, SendAncillaryMessage, SendFlags,
    SocketFlags, SocketType,
};

fn main() {
    // Step 1: a REAL end-to-end SCM_CREDENTIALS round trip over a real
    // AF_UNIX socketpair, using rustix's own public sendmsg/recvmsg API.
    let (send_sock, recv_sock) = socketpair(
        AddressFamily::UNIX,
        SocketType::STREAM,
        SocketFlags::CLOEXEC,
        None,
    )
    .unwrap();

    sockopt::set_socket_passcred(&recv_sock, true).unwrap();

    let ucred = sockopt::socket_peercred(&send_sock).unwrap();
    println!("[step1] real sender ucred = {ucred:?}");

    let msg = SendAncillaryMessage::ScmCredentials(ucred);
    let mut space = [MaybeUninit::uninit(); rustix::cmsg_space!(ScmCredentials(1))];
    let mut send_cmsg_buffer = SendAncillaryBuffer::new(space.as_mut_slice());
    assert!(send_cmsg_buffer.push(msg));

    sendmsg(
        &send_sock,
        &[IoSlice::new(b"x")],
        &mut send_cmsg_buffer,
        SendFlags::empty(),
    )
    .unwrap();

    let mut cmsg_space = [MaybeUninit::uninit(); rustix::cmsg_space!(ScmCredentials(1))];
    let mut data_buf = [0u8; 16];

    {
        let mut recv_cmsg_buffer = RecvAncillaryBuffer::new(cmsg_space.as_mut_slice());
        let result = recvmsg(
            &recv_sock,
            &mut [IoSliceMut::new(&mut data_buf)],
            &mut recv_cmsg_buffer,
            RecvFlags::empty(),
        )
        .unwrap();
        println!("[step1] recvmsg bytes={}", result.bytes);

        match recv_cmsg_buffer.drain().next().unwrap() {
            RecvAncillaryMessage::ScmCredentials(ucred2) => {
                println!("[step1] real parsed ucred = {ucred2:?}");
                assert_eq!(ucred2, ucred);
            }
            _ => panic!("unexpected ancillary message"),
        };
    }

    // Step 2: `cmsg_space` now holds the *genuine* kernel-written
    // cmsghdr+ucred bytes from step 1. Patch only the pid field to 0 (see
    // module doc comment above for why that's the one field this sandbox
    // can't produce via a real cross-PID-namespace kernel delivery), then
    // re-parse via the real public `AncillaryDrain::parse`.
    let align = std::mem::align_of::<libc::cmsghdr>();
    let base_addr = cmsg_space.as_ptr() as usize;
    let aligned_addr = (base_addr + (align - 1)) & align.wrapping_neg();
    let pad = aligned_addr - base_addr;

    let raw: &[u8] = unsafe {
        std::slice::from_raw_parts(
            cmsg_space.as_ptr().cast::<u8>().add(pad),
            cmsg_space.len() - pad,
        )
    };
    let mut patched = raw.to_vec();

    let hdr_ptr = patched.as_ptr().cast::<libc::cmsghdr>();
    let data_ptr = unsafe { libc::CMSG_DATA(hdr_ptr) };
    let payload_offset = data_ptr as usize - patched.as_ptr() as usize;
    println!(
        "[step2] genuine captured bytes = {patched:02x?} (payload_offset={payload_offset})"
    );

    // struct ucred { pid_t pid; uid_t uid; gid_t gid; } - pid is the first
    // 4 bytes of the payload.
    patched[payload_offset..payload_offset + 4].copy_from_slice(&0i32.to_ne_bytes());

    println!("[step2] parsing patched (pid=0) buffer via the real public AncillaryDrain::parse");
    let mut drain = unsafe { AncillaryDrain::parse(&mut patched) };
    match drain.next() {
        Some(RecvAncillaryMessage::ScmCredentials(bad_ucred)) => {
            // Pre-fix: reached, and prints a `Pid` that already violates its
            // own niche invariant (no visible crash in a plain build - see
            // miri_check.rs for where this is actually caught).
            println!("[step2] BUG: constructed UCred with invalid Pid: {bad_ucred:?}");
        }
        None => {
            // Post-fix: the pid=0 payload is recognized as unrepresentable
            // and the ancillary message is safely skipped.
            println!("[step2] FIXED: pid=0 credentials message was safely skipped");
        }
        Some(_) => panic!("unexpected: non-ScmCredentials message"),
    };
}
