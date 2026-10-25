//! Runs the REAL `uname(2)`/`sysinfo(2)`/`getrlimit(2)` syscalls (plain `cargo run`,
//! no Miri) on real, poison-filled memory, to check whether the real kernel/libc actually
//! initializes every byte of the destination struct - independent of whatever Miri's own
//! (possibly incomplete) shim for these functions does.
//!
//! Run with: `cargo run --release --bin poison_byte_real_syscalls`

use std::mem::MaybeUninit;

fn main() {
    unsafe {
        let mut u = MaybeUninit::<libc::utsname>::uninit();
        std::ptr::write_bytes(u.as_mut_ptr(), 0xAA, 1);
        let r = libc::uname(u.as_mut_ptr());
        assert_eq!(r, 0);
        let u = u.assume_init();
        let bytes = std::slice::from_raw_parts(
            &u as *const _ as *const u8,
            std::mem::size_of::<libc::utsname>(),
        );
        println!(
            "uname():     size={}, any 0xAA poison byte remaining: {}",
            bytes.len(),
            bytes.iter().any(|&b| b == 0xAA)
        );

        let mut s = MaybeUninit::<libc::sysinfo>::uninit();
        std::ptr::write_bytes(s.as_mut_ptr(), 0xAA, 1);
        let r = libc::sysinfo(s.as_mut_ptr());
        assert_eq!(r, 0);
        let s = s.assume_init();
        let bytes = std::slice::from_raw_parts(
            &s as *const _ as *const u8,
            std::mem::size_of::<libc::sysinfo>(),
        );
        println!(
            "sysinfo():   size={}, any 0xAA poison byte remaining: {}",
            bytes.len(),
            bytes.iter().any(|&b| b == 0xAA)
        );

        let mut rl = MaybeUninit::<libc::rlimit>::uninit();
        std::ptr::write_bytes(rl.as_mut_ptr(), 0xAA, 1);
        let r = libc::getrlimit(libc::RLIMIT_NOFILE, rl.as_mut_ptr());
        assert_eq!(r, 0);
        let rl = rl.assume_init();
        let bytes = std::slice::from_raw_parts(
            &rl as *const _ as *const u8,
            std::mem::size_of::<libc::rlimit>(),
        );
        println!(
            "getrlimit(): size={}, any 0xAA poison byte remaining: {}",
            bytes.len(),
            bytes.iter().any(|&b| b == 0xAA)
        );
    }
}
