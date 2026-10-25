//! Real end-to-end repro for https://github.com/bytecodealliance/rustix/issues/1689.
//!
//! This crate is a genuine downstream consumer of `rustix` built with exactly the feature
//! selection the issue reports (`default-features = false, features = ["std", "net"]`) - see
//! `Cargo.toml`. On the buggy commit this fails to even *compile* (`error[E0433]: cannot find
//! `timespec` in `crate``, 6 times), because both backends' `sockopt` modules reference
//! `crate::timespec` unconditionally even though `net` does not gate that module in.
//!
//! Beyond compiling, this also exercises the real affected public API end-to-end against a real
//! socket (`set_socket_timeout` / `socket_timeout`, the functions whose `SO_RCVTIMEO`/
//! `SO_SNDTIMEO` support is what actually needs `crate::timespec`), so a fix is verified not just
//! to compile but to behave correctly at runtime too.

use rustix::net::sockopt::{set_socket_timeout, socket_timeout, Timeout};
use rustix::net::{AddressFamily, SocketType};
use std::time::Duration;

fn main() {
    let sock = rustix::net::socket(AddressFamily::INET, SocketType::DGRAM, None)
        .expect("failed to create a real AF_INET/SOCK_DGRAM socket");

    let want = Duration::from_millis(250);
    set_socket_timeout(&sock, Timeout::Recv, Some(want)).expect("set_socket_timeout(Recv) failed");
    let got = socket_timeout(&sock, Timeout::Recv)
        .expect("socket_timeout(Recv) failed")
        .expect("expected Some(duration) back, got None");

    assert_eq!(got, want, "round-tripped socket timeout did not match what was set");

    println!("OK: rustix built with default-features=false, features=[\"std\",\"net\"] compiles and set_socket_timeout/socket_timeout round-trip correctly ({got:?})");
}
