//! Calls rustix's real, public `rustix::termios::tcgetattr()` on stdin's fd.
//!
//! On `target_os = "nto"` (QNX) specifically, the non-Linux generic path in
//! `src/backend/libc/termios/syscalls.rs` does:
//!
//!   let mut result = MaybeUninit::<Termios>::uninit();
//!   ret(c::tcgetattr(borrowed_fd(fd), result.as_mut_ptr().cast()))?;
//!   Ok(result.assume_init())
//!
//! `Termios` carries a `pub(crate) __reserved: [ffi::c_uint; 3]` field only on `target_os =
//! "nto"`, mirroring QNX's own `struct termios`'s `reserved[3]` field (confirmed from QNX's
//! official docs and from the real QNX6 libc source, see RESULT.md/output.txt) - never
//! explicitly initialized anywhere in rustix's own code before this call, unlike the sibling
//! `futex_waitv::__reserved` field which rustix zeroes explicitly in its own constructor
//! (src/thread/futex.rs).
//!
//! This binary demonstrates a genuine, current (2026-09-29 nightly) tooling gap: Miri has no
//! foreign-function shim for `tcgetattr` on OS "nto" at all, so it cannot be used to observe
//! whether the real syscall leaves `__reserved` uninitialized - it fails before reaching the
//! question, the same outcome candidate 050 already found on an earlier session. This is NOT a
//! repro of the suspected bug - no execution against it was ever possible in this environment
//! (no QNX hardware/emulator, no Miri shim). See RESULT.md for why Status is "Not reproducible"
//! despite the strong circumstantial case laid out there.
//!
//!   RUSTFLAGS="--cfg rustix_no_linux_raw" cargo miri run --target x86_64-pc-nto-qnx710 --bin tcgetattr_public_api

fn main() {
    let fd = unsafe { rustix::fd::BorrowedFd::borrow_raw(0) };
    match rustix::termios::tcgetattr(fd) {
        Ok(t) => println!("tcgetattr ok, iflag bits = {:?}", t.input_modes),
        Err(e) => println!("tcgetattr failed: {:?}", e),
    }
}
