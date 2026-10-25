//! Real end-to-end exercise of rustix's public API, cross-interpreted by Miri
//! for `--target x86_64-unknown-netbsd`, for the NetBSD-gated code paths that
//! Miri's netbsd shims actually support (plain, non-symbol-versioned libc
//! functions: open/read/write/pipe/poll). This is the "it works" half of the
//! sweep - see `unsupported_sweep.rs` for the half that Miri can't execute at
//! all on this target.
use rustix::event::{poll, PollFd, PollFlags};
use rustix::fd::AsFd;
use std::io::Write;
use std::os::fd::{AsRawFd, FromRawFd};

fn main() {
    // Real file I/O through rustix-adjacent std APIs.
    let path = std::env::temp_dir().join("rustix_netbsd_miri_io_sweep.txt");
    let mut f = std::fs::File::create(&path).unwrap();
    f.write_all(b"hello").unwrap();
    drop(f);
    let _ = std::fs::remove_file(&path);
    println!("file create/write/remove: ok");

    // Real pipe() + poll() through rustix's own public API - this exercises
    // the NetBSD-gated `ppoll`-weak-symbol-with-plain-`poll`-fallback logic in
    // src/backend/libc/event/syscalls.rs (the only block in event/syscalls.rs
    // gated in part on `target_os = "netbsd"`).
    let (a, b) = rustix::pipe::pipe().unwrap();
    let mut bf = unsafe { std::fs::File::from_raw_fd(b.as_raw_fd()) };
    std::mem::forget(b);
    bf.write_all(b"x").unwrap();

    let afd = a.as_fd();
    let mut fds = [PollFd::new(&afd, PollFlags::IN)];
    let n = poll(&mut fds, None).unwrap();
    println!(
        "pipe + poll (netbsd ppoll-weak-symbol fallback path): n={}, revents={:?}",
        n,
        fds[0].revents()
    );

    // Pure computation, no syscall at all: NetBSD's makedev/major/minor are
    // plain const fns in the vendored libc crate (no `extern "C"`), so this
    // exercises rustix's own wrapper logic directly, byte for byte, under
    // Miri - no shim gap possible here.
    let maj = 0xffu32; // 8-bit field, deliberately at its max
    let min = 0xf_ffffu32; // 20-bit field, deliberately at its max
    let dev = rustix::fs::makedev(maj, min);
    let got_maj = rustix::fs::major(dev);
    let got_min = rustix::fs::minor(dev);
    println!(
        "makedev(0x{:x}, 0x{:x}) = 0x{:x} -> major=0x{:x} minor=0x{:x}",
        maj, min, dev, got_maj, got_min
    );
    assert_eq!(got_maj, maj, "major() did not round-trip");
    assert_eq!(got_min, min, "minor() did not round-trip");
    println!("makedev/major/minor round-trip: ok");
}
