// Demonstrates rustix's fs::statvfs()/fstatvfs() f_favail bug via the real, unmodified
// end-to-end path: a real libc statvfs()/fstatvfs() syscall against a real filesystem, through
// rustix's real public API (forced to the libc backend with RUSTFLAGS=--cfg=rustix_use_libc so
// this also runs on x86_64 Linux, not just the non-Linux targets this backend ships on by
// default).
//
// Every accessible filesystem in this sandbox reports f_ffree == f_favail (Linux's own kernel
// statfs/statfs64 API has no separate "available to unprivileged processes" inode concept, so
// glibc's own statvfs()/statvfs64() wrapper always copies one into the other) - so the bug's
// *effect* can't be observed from an unmodified real syscall alone here. To demonstrate the
// mechanism without fabricating any application-level data, this repro loads a small LD_PRELOAD
// shim (shim.c) that passes every call straight through to the real libc statvfs64/fstatvfs64
// (via a real dlsym(RTLD_NEXT, ...) lookup - not reimplemented) and only overwrites the
// `f_favail` field of the real result, simulating a real OS/filesystem that implements POSIX's
// documented distinction between `f_ffree` (total free inodes) and `f_favail` (inodes available
// to unprivileged processes, which every single vendored `libc` crate platform module defines as
// a genuinely separate field - see RESULT.md).
//
// IMPORTANT: on this glibc/x86_64 target, rustix's own `c::statvfs`/`c::fstatvfs` are literally
// `libc::statvfs64`/`libc::fstatvfs64` (see `src/backend/libc/c.rs`), NOT the plain
// `libc::statvfs`/`libc::fstatvfs` symbols a naive caller might reach for - so the shim must
// intercept the `64`-suffixed symbols too, or rustix's own call silently bypasses it while a
// direct `libc::statvfs()` call in the same process does not (this repro calls the plain
// symbols itself, for contrast/sanity-checking, and the shim handles both pairs).
fn main() {
    let path = std::ffi::CString::new(".").unwrap();
    let mut raw: libc::statvfs = unsafe { std::mem::zeroed() };
    let rc = unsafe { libc::statvfs(path.as_ptr(), &mut raw) };
    assert_eq!(rc, 0, "raw libc::statvfs failed");

    let r = rustix::fs::statvfs(".").expect("rustix statvfs failed");

    println!("raw.f_ffree  = {}", raw.f_ffree);
    println!("raw.f_favail = {}", raw.f_favail);
    println!("rustix StatVfs.f_favail (via statvfs)  = {}", r.f_favail);

    let report = |label: &str, raw_ffree: u64, raw_favail: u64, out_favail: u64| {
        if raw_favail == raw_ffree {
            println!(
                "{label}: (no LD_PRELOAD shim active, or this fs genuinely reports them equal)"
            );
        } else if out_favail == raw_ffree {
            println!(
                "{label}: BUG REPRODUCED: rustix ignored the real f_favail ({raw_favail}) and substituted f_ffree ({raw_ffree})"
            );
        } else if out_favail == raw_favail {
            println!("{label}: CORRECT: rustix's output matches the real f_favail field");
        } else {
            println!("{label}: UNEXPECTED: rustix's output matches neither field");
        }
    };

    report(
        "statvfs",
        raw.f_ffree as u64,
        raw.f_favail as u64,
        r.f_favail,
    );

    // Same conversion function (`libc_statvfs_to_statvfs`) is shared by the `fstatvfs` entry
    // point - exercise it too, through a real open file descriptor.
    let file = std::fs::File::open(".").expect("open \".\" failed");
    use std::os::unix::io::AsRawFd;
    let mut raw2: libc::statvfs = unsafe { std::mem::zeroed() };
    let rc2 = unsafe { libc::fstatvfs(file.as_raw_fd(), &mut raw2) };
    assert_eq!(rc2, 0, "raw libc::fstatvfs failed");
    let r2 = rustix::fs::fstatvfs(&file).expect("rustix fstatvfs failed");
    println!("raw2.f_ffree  = {}", raw2.f_ffree);
    println!("raw2.f_favail = {}", raw2.f_favail);
    println!("rustix StatVfs.f_favail (via fstatvfs) = {}", r2.f_favail);
    report(
        "fstatvfs",
        raw2.f_ffree as u64,
        raw2.f_favail as u64,
        r2.f_favail,
    );
}
