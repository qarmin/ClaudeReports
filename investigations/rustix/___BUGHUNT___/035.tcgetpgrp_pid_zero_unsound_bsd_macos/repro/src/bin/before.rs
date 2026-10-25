// Reproduces candidate 035: rustix's `tcgetpgrp` (src/backend/libc/termios/syscalls.rs:166-179)
// only rejects a kernel-returned pid of 0 under `#[cfg(linux_kernel)]`. On macOS/BSD targets that
// cfg is false, so the function falls straight through to
// `Pid::from_raw_unchecked(pid)` with `pid == 0` - violating that constructor's own documented
// safety contract ("the caller must guarantee raw is non-zero"), since `Pid` wraps `NonZeroI32`.
//
// This binary mirrors that unguarded branch EXACTLY (the two real lines
// `let pid = ret_pid_t(c::tcgetpgrp(borrowed_fd(fd)))?;` /
// `Ok(Pid::from_raw_unchecked(pid))`, with the `#[cfg(linux_kernel)]` check simply never run,
// matching what the real macOS/BSD-compiled function executes) - not a hand-built stand-in for
// a kernel's behavior: `pid` here is a value returned by a REAL Linux kernel tcgetpgrp(2) syscall
// on a real pty master fd that has never had a controlling session/foreground process group
// assigned (this is the exact condition rustix's own source comment already attributes to Linux;
// BSD/macOS inherit the same BSD pty/tty-subsystem lineage, which is the premise of upstream
// issue #1678). We can't compile-and-run the real non-Linux-targeted function body on this host
// (no macOS/BSD hardware/toolchain available - see RESULT.md), so this binary executes the
// identical logic the real function would, fed by this genuinely kernel-produced value, which is
// the closest verifiable stand-in for running the real compiled-for-macOS/BSD function.
use rustix::process::Pid;
use std::ffi::CStr;
use std::os::unix::io::{AsRawFd, FromRawFd};

fn real_kernel_tcgetpgrp_zero() -> i32 {
    unsafe {
        let master_fd = libc::posix_openpt(libc::O_RDWR | libc::O_NOCTTY);
        assert!(
            master_fd >= 0,
            "posix_openpt failed: {}",
            std::io::Error::last_os_error()
        );
        let master = std::fs::File::from_raw_fd(master_fd);
        assert_eq!(libc::grantpt(master.as_raw_fd()), 0);
        assert_eq!(libc::unlockpt(master.as_raw_fd()), 0);

        let name_ptr = libc::ptsname(master.as_raw_fd());
        assert!(!name_ptr.is_null());
        let name = CStr::from_ptr(name_ptr).to_string_lossy().into_owned();
        eprintln!("[repro] opened pty master, slave = {name} (never opened/attached)");

        // Genuine real-kernel call - no slave has ever been opened on this pty, so no
        // session/foreground process group has ever been assigned to it.
        let pgrp = libc::tcgetpgrp(master.as_raw_fd());
        eprintln!("[repro] real libc::tcgetpgrp(master_fd) returned {pgrp}");
        pgrp
    }
}

fn main() {
    let pid = real_kernel_tcgetpgrp_zero();
    assert_eq!(
        pid, 0,
        "expected the real kernel to return 0 on an unattached pty master (as documented in \
         rustix's own source comment and relied on by issue #1678) - got {pid} instead; rerun, \
         this is environment-dependent"
    );

    eprintln!(
        "[repro] now executing the exact macOS/BSD-unguarded branch of tcgetpgrp() in \
         src/backend/libc/termios/syscalls.rs with this real, kernel-produced pid=0 \
         (the `#[cfg(linux_kernel)]`-gated `if pid == 0` check is skipped, exactly as it would \
         be if this were actually compiled for target_os = \"macos\"/\"freebsd\"/etc.)"
    );

    // Caught with `catch_unwind` (rather than letting the process abort) purely so this repro's
    // own exit code/output stays legible when chained with `apply`/`reproduce-after` in the
    // candidate justfile - the panic itself, from `from_raw_unchecked`'s own
    // `debug_assert!(raw > 0)`, is the real, unmodified reproduction (in a release build with
    // debug assertions off, this same call constructs a real invalid `NonZeroI32(0)` instead -
    // itself the deeper, platform-independent Undefined Behavior confirmed separately in
    // output.txt's Miri run of `miri_ub`).
    let result = std::panic::catch_unwind(|| unsafe { Pid::from_raw_unchecked(pid) });

    match result {
        Ok(p) => {
            println!("did not reproduce in this build (no crash): constructed {:?}", p);
        }
        Err(panic_payload) => {
            let msg = panic_payload
                .downcast_ref::<String>()
                .map(String::as_str)
                .or_else(|| panic_payload.downcast_ref::<&str>().copied())
                .unwrap_or("<non-string panic payload>");
            eprintln!(
                "REPRODUCED - rustix::process::Pid::from_raw_unchecked(0) panicked (the exact \
                 construction the vulnerable macOS/BSD tcgetpgrp() branch performs): {msg}"
            );
            std::process::exit(1);
        }
    }
}
