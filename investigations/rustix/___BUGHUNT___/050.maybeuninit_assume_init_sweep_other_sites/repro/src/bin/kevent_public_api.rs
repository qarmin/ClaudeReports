//! Calls rustix's real, public `rustix::event::kqueue::{kqueue, kevent}` (BSD/macOS).
//! Demonstrates Miri has no shim for `kqueue()` at all on OS "macos" - a tooling gap, not
//! independent confirmation either way for the suspected bug at this call site.
//!
//!   MIRIFLAGS="-Zmiri-disable-isolation" cargo miri run --target x86_64-apple-darwin --bin kevent_public_api

fn main() {
    let kq = rustix::event::kqueue::kqueue().expect("kqueue");
    let mut buf: Vec<std::mem::MaybeUninit<rustix::event::kqueue::Event>> = Vec::with_capacity(4);
    buf.resize_with(4, std::mem::MaybeUninit::uninit);
    let timeout = Some(std::time::Duration::from_millis(1));
    unsafe {
        match rustix::event::kqueue::kevent(&kq, &[], &mut buf[..], timeout) {
            Ok(_n) => println!("kevent ok"),
            Err(e) => println!("kevent err: {:?}", e),
        }
    }
}
