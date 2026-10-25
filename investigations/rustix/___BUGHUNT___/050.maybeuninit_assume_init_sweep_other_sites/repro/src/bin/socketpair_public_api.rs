//! Calls rustix's real, public `rustix::net::socketpair()`. Runs cleanly under Miri (both
//! native Linux and cross-interpreted for x86_64-apple-darwin), unlike `uname`/`sysinfo` -
//! Miri's own socketpair shim does fully initialize both returned file descriptors.
//!
//!   MIRIFLAGS="-Zmiri-disable-isolation" cargo miri run --bin socketpair_public_api
//!   MIRIFLAGS="-Zmiri-disable-isolation" cargo miri run --target x86_64-apple-darwin --bin socketpair_public_api

fn main() {
    match rustix::net::socketpair(
        rustix::net::AddressFamily::UNIX,
        rustix::net::SocketType::STREAM,
        rustix::net::SocketFlags::empty(),
        None,
    ) {
        Ok((_a, _b)) => println!("socketpair ok"),
        Err(e) => println!("socketpair err: {:?}", e),
    }
}
