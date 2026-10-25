//! Calls rustix's real, public `rustix::system::uname()`.
//!
//! Run under Miri to see the (misleading - see RESULT.md) UB signal:
//!   MIRIFLAGS="-Zmiri-disable-isolation" cargo miri run --target x86_64-apple-darwin --bin uname_public_api
//!   MIRIFLAGS="-Zmiri-disable-isolation" cargo miri run --bin uname_public_api   # native linux, same UB
//!
//! Run normally (no Miri) to see it succeed cleanly against the real syscall:
//!   cargo run --bin uname_public_api

fn main() {
    let u = rustix::system::uname();
    println!("uname ok: sysname={:?}", u.sysname());
}
