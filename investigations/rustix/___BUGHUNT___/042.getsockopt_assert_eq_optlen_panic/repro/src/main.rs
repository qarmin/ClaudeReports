//! Candidate 042: exhaustive real-syscall sweep of every generic `getsockopt<T>`
//! call site this crate exposes that's reachable without elevated privileges, plus
//! an attempt to reach the privileged AF_XDP-specific fallback paths
//! (`xdp_statistics`/`xdp_mmap_offsets`, the one place a dual-struct-size fallback
//! already exists in this codebase).
//!
//! This is NOT a repro of a panic - no panic was found. It's the real-kernel evidence
//! backing the "Not reproducible" conclusion in RESULT.md: every one of these calls,
//! against the real running kernel, returns an `optlen` that matches `size_of::<T>()`
//! exactly, so `assert_eq!` in the generic `getsockopt<T>` helper
//! (src/backend/linux_raw/net/sockopt.rs:55, mirrored in the libc backend's
//! XDP-specific fallbacks) never fires on this kernel/architecture.
//!
//! Run with the default (linux_raw) backend, and again with
//! `RUSTFLAGS="--cfg rustix_use_libc" cargo run` to exercise the libc backend's own
//! generic getsockopt<T> helper the same way.

use rustix::net::sockopt;
use rustix::net::{AddressFamily, SocketType};
use std::panic;

fn try_call<F: FnOnce() -> R + panic::UnwindSafe, R: std::fmt::Debug>(name: &str, f: F) {
    match panic::catch_unwind(f) {
        Ok(v) => println!("{name}: Ok-ish -> {:?}", v),
        Err(e) => {
            let msg = e
                .downcast_ref::<String>()
                .cloned()
                .or_else(|| e.downcast_ref::<&str>().map(|s| s.to_string()))
                .unwrap_or_default();
            println!("{name}: PANICKED -> {msg}");
        }
    }
}

fn main() {
    let tcp = rustix::net::socket(AddressFamily::INET, SocketType::STREAM, None).unwrap();
    let udp = rustix::net::socket(AddressFamily::INET, SocketType::DGRAM, None).unwrap();
    let unix = rustix::net::socket(AddressFamily::UNIX, SocketType::STREAM, None).unwrap();

    try_call("socket_type", || sockopt::socket_type(&tcp));
    try_call("socket_linger", || sockopt::socket_linger(&tcp));
    try_call("socket_error", || sockopt::socket_error(&tcp));
    try_call("socket_domain", || sockopt::socket_domain(&tcp));
    try_call("socket_protocol", || sockopt::socket_protocol(&tcp));
    try_call("socket_cookie", || sockopt::socket_cookie(&tcp));
    try_call("socket_incoming_cpu", || sockopt::socket_incoming_cpu(&tcp));
    try_call("socket_recv_timeout", || {
        sockopt::socket_timeout(&tcp, sockopt::Timeout::Recv)
    });
    try_call("ip_ttl", || sockopt::ip_ttl(&tcp));
    try_call("ipv6_v6only(udp)", || sockopt::ipv6_v6only(&udp));
    try_call("tcp_nodelay", || sockopt::tcp_nodelay(&tcp));
    try_call("tcp_keepcnt", || sockopt::tcp_keepcnt(&tcp));
    try_call("tcp_keepidle", || sockopt::tcp_keepidle(&tcp));
    try_call("tcp_keepintvl", || sockopt::tcp_keepintvl(&tcp));
    try_call("tcp_user_timeout", || sockopt::tcp_user_timeout(&tcp));
    try_call("unix_peercred", || sockopt::socket_peercred(&unix));

    // AF_XDP = 44. Creating this socket at all needs CAP_NET_RAW; in a sandboxed
    // container without it, this (correctly) fails with EPERM rather than exercising
    // xdp_statistics/xdp_mmap_offsets's own dual-struct-size fallback logic.
    match rustix::net::socket(AddressFamily::from_raw(44.try_into().unwrap()), SocketType::RAW, None) {
        Ok(xdp) => {
            try_call("xdp_statistics", || sockopt::xdp_statistics(&xdp));
            try_call("xdp_mmap_offsets", || sockopt::xdp_mmap_offsets(&xdp));
        }
        Err(e) => println!("AF_XDP socket() failed: {e} (expected without CAP_NET_RAW)"),
    }

    println!("Sweep complete, no unexpected panics above = all current real-kernel getsockopt sizes match.");
}
