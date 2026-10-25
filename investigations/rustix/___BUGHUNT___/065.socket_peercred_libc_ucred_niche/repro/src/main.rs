//! Real end-to-end trigger for candidate 065: `rustix::net::sockopt::socket_peercred`
//! (libc backend) constructs an invalid `Pid` (NonZeroI32 niche) when the real kernel
//! legitimately reports `pid == 0` for `SO_PEERCRED`.
//!
//! No faked peer/second participant is needed here: the single real external input is
//! the kernel's own `getsockopt(SO_PEERCRED)` response to a real, genuinely-created
//! AF_UNIX `SOCK_STREAM` socket that has never been `connect()`'d/`accept()`'d - this is
//! real, standard, documented Linux behavior (not something this repro invents), exercised
//! entirely through rustix's own real public API (`rustix::net::socket`,
//! `rustix::net::sockopt::socket_peercred`, `rustix::net::socketpair`).

use rustix::net::sockopt::socket_peercred;
use rustix::net::{socket, socketpair, AddressFamily, SocketFlags, SocketType};

fn raw_libc_cross_check(fd: &impl std::os::fd::AsRawFd) {
    unsafe {
        let mut cred: libc::ucred = std::mem::zeroed();
        let mut len = std::mem::size_of::<libc::ucred>() as libc::socklen_t;
        let r = libc::getsockopt(
            fd.as_raw_fd(),
            libc::SOL_SOCKET,
            libc::SO_PEERCRED,
            &mut cred as *mut _ as *mut libc::c_void,
            &mut len,
        );
        println!(
            "  [raw libc cross-check] getsockopt(SO_PEERCRED) -> r={} errno={} pid={} uid={} gid={}",
            r,
            std::io::Error::last_os_error(),
            cred.pid,
            cred.uid,
            cred.gid
        );
    }
}

fn main() {
    println!("== Case 1: real, never-connected AF_UNIX SOCK_STREAM socket ==");
    let fd = socket(AddressFamily::UNIX, SocketType::STREAM, None)
        .expect("socket() should succeed");
    raw_libc_cross_check(&fd);

    match socket_peercred(&fd) {
        Ok(cred) => {
            println!("  rustix::socket_peercred -> Ok: {cred:?}");
        }
        Err(e) => {
            println!("  rustix::socket_peercred -> Err: {e:?}");
        }
    }

    println!();
    println!("== Case 2 (sanity check): real connected pair, legitimate nonzero pid ==");
    let (a, _b) = socketpair(
        AddressFamily::UNIX,
        SocketType::STREAM,
        SocketFlags::empty(),
        None,
    )
    .expect("socketpair() should succeed");
    raw_libc_cross_check(&a);
    match socket_peercred(&a) {
        Ok(cred) => println!("  rustix::socket_peercred -> Ok: {cred:?}"),
        Err(e) => println!("  rustix::socket_peercred -> Err: {e:?}"),
    }
}
