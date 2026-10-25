// Standalone reproduction for candidate 025: `rustix::fs::sendfile` is unreachable when
// cross-compiling for Android, even though Android runs the Linux kernel and bionic provides
// `sendfile64` (the same syscall rustix's libc backend already calls on "regular" Linux).
//
// This is a compile-time missing-API bug, not a runtime crash: the module declaration, the
// function itself (in one backend), and the public re-export are all gated on the literal
// `#[cfg(target_os = "linux")]`, which excludes `target_os = "android"` - instead of the crate's
// own `linux_kernel` cfg alias (build.rs: `os == "android" || os == "linux"`), which every
// sibling Linux-kernel-specific item in the same file already uses.
//
// This file itself compiles identically regardless of whether the bug is present - the bug only
// shows up when *type-checking this exact file* for an Android target. See repro/README or the
// candidate's own justfile for how `reproduce-before`/`reproduce-after` invoke
// `cargo check --target x86_64-linux-android` against it (pristine vs. patched clone) rather than
// `cargo run`, since an Android binary can't run directly on this host anyway and the bug is
// visible purely from whether the public symbol resolves.

use rustix::fd::BorrowedFd;

/// Calls the real, public `rustix::fs::sendfile` API exactly as a genuine caller would.
/// On a pristine (unpatched) checkout, cross-type-checking this for an Android target fails with
/// `error[E0425]: cannot find value/function \`sendfile\` in module \`rustix::fs\`` - the function
/// was configured out entirely, not merely behaving incorrectly at runtime.
fn use_sendfile(out_fd: BorrowedFd<'_>, in_fd: BorrowedFd<'_>) -> std::io::Result<usize> {
    Ok(rustix::fs::sendfile(out_fd, in_fd, None, 4096)?)
}

fn main() {
    let _ = use_sendfile;
    println!("REPRO: rustix::fs::sendfile resolved and type-checked for this target");
}
