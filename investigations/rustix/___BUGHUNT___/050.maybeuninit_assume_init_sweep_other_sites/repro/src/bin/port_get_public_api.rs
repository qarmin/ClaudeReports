//! Calls rustix's real, public `rustix::event::port::{create, get}` (Solaris/illumos).
//! Demonstrates Miri has no shim for `port_create()` at all on OS "illumos" - a tooling gap,
//! not independent confirmation either way for the suspected bug at this call site
//! (`src/backend/libc/event/syscalls.rs::port_get`'s `MaybeUninit::<c::port_event>::uninit()`).
//!
//!   MIRIFLAGS="-Zmiri-disable-isolation" cargo miri run --target x86_64-unknown-illumos --bin port_get_public_api

fn main() {
    let port = rustix::event::port::create().expect("port_create");
    match rustix::event::port::get(
        &port,
        Some(&rustix::event::Timespec {
            tv_sec: 0,
            tv_nsec: 1_000_000,
        }),
    ) {
        Ok(e) => println!("port_get ok: events={}", e.events()),
        Err(e) => println!("port_get err: {:?}", e),
    }
}
