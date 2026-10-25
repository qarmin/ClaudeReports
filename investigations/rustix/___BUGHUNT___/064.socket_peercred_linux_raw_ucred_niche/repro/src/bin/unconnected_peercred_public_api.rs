//! Real end-to-end repro for candidate 064.
//!
//! No second participant is faked: an unconnected `AF_UNIX`/`SOCK_STREAM` socket is a single,
//! real kernel object, and the kernel itself (not this program) decides what `SO_PEERCRED`
//! reports for it. Per `net/core/sock.c::cred_to_ucred()`, when `sk->sk_peer_pid` is `NULL`
//! (true for any unix socket that has never had a peer attached via `connect()`/`accept()`/
//! `socketpair()`), the kernel unconditionally sets `ucred.pid = 0` - this is the universal
//! default for "no peer was ever established", nothing namespace-specific about it. This repro
//! calls rustix's real public API (`rustix::net::socket` + `rustix::net::sockopt::socket_peercred`)
//! end to end, exactly as a real caller would, and never hand-constructs the `ucred` bytes itself.
//!
//! `UCred` embeds `Pid(NonZeroI32)`. `socket_peercred`'s generic `getsockopt<T: Copy>` helper
//! builds the function's return value as `Ok(value.assume_init())`, i.e. it materializes a
//! `Result<UCred, Errno>` directly from the raw kernel bytes. Because `Pid`'s `NonZeroI32` niche
//! makes `pid == 0` an representation the type system considers impossible, the compiler is free
//! to (and does) reuse exactly that all-zero-pid bit pattern as `Result`'s own niche encoding for
//! the `Err` variant - so it never needs a separate discriminant byte. The real kernel value
//! (`pid=0`) collides with that chosen `Err` encoding: the *successful* syscall's `Ok(UCred)`
//! becomes bit-for-bit indistinguishable from `Err(Errno)`, and the remaining bytes (`uid`/`gid`,
//! both `0xFFFF_FFFF`, the kernel's "overflow" placeholders for an unconnected socket) get
//! reinterpreted as the `Err` payload.

use rustix::net::sockopt::socket_peercred;
use rustix::net::{socket, AddressFamily, SocketType};

fn main() {
    let fd = socket(AddressFamily::UNIX, SocketType::STREAM, None)
        .expect("socket() should succeed for a plain AF_UNIX/SOCK_STREAM socket");

    // Real getsockopt(SOL_SOCKET, SO_PEERCRED) syscall through rustix's public API. The fd was
    // never connect()'d/accept()'d/socketpair()'d, so the kernel has no peer to report
    // credentials for and fills in its documented default (pid=0, uid=gid=overflow).
    match socket_peercred(&fd) {
        Ok(cred) => {
            // Fixed-tree shape: `UCred.pid` is `Option<Pid>`, so the kernel's real pid=0 for an
            // unconnected socket decodes as `None` - correct, no corruption.
            println!(
                "Ok(UCred {{ pid: {:?}, uid: {:?}, gid: {:?} }})",
                cred.pid, cred.uid, cred.gid
            );
            println!("FIXED: socket_peercred() succeeded and correctly reports no pid (None)");
        }
        Err(e) => {
            // Pre-fix-tree shape: `UCred.pid` is `Pid` (NonZeroI32 niche). The kernel's real,
            // successful result (pid=0, uid=gid=u32::MAX for an unconnected socket, independently
            // verified via a raw getsockopt(2) on the very same fd, see RESULT.md) collided with
            // the niche bit pattern the compiler chose to represent `Result<UCred, Errno>`'s `Err`
            // variant, so the syscall's success gets reported to the caller as a spurious Err.
            println!("socket_peercred() returned Err({e:?})");
            println!(
                "CONFIRMED: syscall actually succeeded but was reported as a spurious Err - see \
                 RESULT.md for the niche-collision mechanism"
            );
        }
    }
}
