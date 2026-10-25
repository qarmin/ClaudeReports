// Standalone reproduction for candidate 058: on Apple targets, `rustix::fs::getpath`
// (src/backend/libc/fs/syscalls.rs, `getpath`) does `buf.iter().position(|&c| c == 0).unwrap()`
// on the raw bytes the real `fcntl(fd, F_GETPATH, ...)` syscall wrote into a `PATH_MAX`-sized
// buffer, trusting the kernel to always leave a NUL terminator somewhere within those bytes.
// Apple's own `fcntl(2)` manual page only promises the buffer must be `MAXPATHLEN` (== `PATH_MAX`)
// *or greater* - it never promises the kernel leaves room to spare for the terminator once the
// path itself is at or near that length.
//
// This crate is `#[cfg(all(apple, feature = "alloc"))]`-gated, so it cannot be built or executed
// for a non-Apple host at all - there is no way to reach this code on plain x86_64 Linux. This
// sandboxed container also has no macOS SDK/cross-linker (`xcrun` is absent, cross-compiling to
// `x86_64-apple-darwin` only ever produces an rlib, never a linked executable) and no way to run a
// real Darwin binary (qemu-user here only emulates foreign-CPU Linux-ABI syscalls, not XNU's
// distinct syscall ABI) - see RESULT.md's "Fully reproducible" section for the exact commands that
// established this. A real macOS kernel's actual `F_GETPATH` response is therefore the
// unavailable "second participant" for this bug (the same shape candidate 032 hit for a FUSE
// daemon, and candidates 038/039 hit for real Apple hardware in general).
//
// Given that, this repro drives the REAL, unmodified, `cfg(apple)`-gated `getpath` function via
// Miri's cross-interpretation (`cargo miri run --target x86_64-apple-darwin`, which interprets
// MIR directly and does not need to natively execute Darwin code) against a worktree whose
// `mock.patch` is applied first. `mock.patch` intercepts only the single lowest-level `fcntl`
// syscall call, for two magic raw fd values, simulating the pathological/boundary kernel
// responses; every other fd still takes the real, unmodified syscall. Everything else exercised -
// `rustix::fs::getpath`'s real public API and the real, unmodified
// `position(...).unwrap()`/`truncate`/`CString::new(...).unwrap()` logic - is exactly the source
// under test, compiled for the real `cfg(apple)` target.
use std::os::fd::{AsFd, BorrowedFd};

fn describe_payload(payload: &(dyn std::any::Any + Send)) -> String {
    if let Some(s) = payload.downcast_ref::<String>() {
        s.clone()
    } else if let Some(s) = payload.downcast_ref::<&str>() {
        (*s).to_string()
    } else {
        "<non-string panic payload>".to_string()
    }
}

fn check(label: &str, raw_fd: i32) {
    // SAFETY: never dereferenced as a real fd - mock.patch intercepts these exact raw values
    // before any syscall is made on them.
    let fake_fd: BorrowedFd<'static> = unsafe { BorrowedFd::borrow_raw(raw_fd) };

    let result = std::panic::catch_unwind(|| rustix::fs::getpath(fake_fd.as_fd()));

    match result {
        Ok(Ok(cstr)) => {
            println!("[{label}] did not reproduce - getpath succeeded with {:?}", cstr);
        }
        Ok(Err(e)) => {
            println!("[{label}] did not reproduce - getpath returned a clean error: {:?}", e);
        }
        Err(payload) => {
            eprintln!("[{label}] REPRODUCED - panicked: {}", describe_payload(&*payload));
        }
    }
}

fn main() {
    // Full PATH_MAX buffer, every byte non-zero (no NUL anywhere) - the pathological case.
    check("full-nonzero-buffer", 999_999);
    // Boundary variant: every byte non-zero except the very last, which the kernel does write
    // as the NUL terminator - the legitimate maximal case, must NOT panic.
    check("boundary-last-byte-nul", 999_998);
}
