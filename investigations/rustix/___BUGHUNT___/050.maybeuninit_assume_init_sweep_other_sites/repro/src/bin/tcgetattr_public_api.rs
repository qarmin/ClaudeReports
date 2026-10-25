//! Calls rustix's real, public `rustix::termios::tcgetattr()` on stdin's fd. On
//! `target_os = "nto"` (QNX), the non-Linux generic path casts straight into rustix's own
//! `Termios`, which carries a `pub(crate) __reserved: [ffi::c_uint; 3]` field mirroring QNX's
//! own `libc::termios::__reserved` - never explicitly initialized anywhere in rustix's own
//! code, relying entirely on QNX's `tcgetattr()` to fill it in. Demonstrates Miri has no shim
//! for `tcgetattr` at all on OS "nto" - a tooling gap, not independent confirmation either way.
//!
//!   MIRIFLAGS="-Zmiri-disable-isolation" cargo miri run --target x86_64-pc-nto-qnx710 --bin tcgetattr_public_api

fn main() {
    let fd = unsafe { rustix::fd::BorrowedFd::borrow_raw(0) };
    match rustix::termios::tcgetattr(fd) {
        Ok(t) => println!("tcgetattr ok, iflag bits = {:?}", t.input_modes),
        Err(e) => println!("tcgetattr failed: {:?}", e),
    }
}
