// Candidate 080: rustix's `tcgetsid()` (src/backend/libc/termios/syscalls.rs:346-351 and
// src/backend/linux_raw/termios/syscalls.rs:249-256) calls `Pid::from_raw_unchecked(pid)` on the
// raw `pid_t` returned by the real `TIOCGSID` ioctl with NO check that `pid != 0`, unlike its
// sibling `tcgetpgrp` (candidate 035), which does check (unconditionally in the linux_raw
// backend, and under `#[cfg(linux_kernel)]` in the libc backend).
//
// The real Linux kernel's own `tiocgsid()` (drivers/tty/tty_jobctrl.c) computes
// `sid = pid_vnr(real_tty->ctrl.session)`, and `pid_vnr`/`pid_nr_ns` (kernel/pid.c) returns 0
// whenever the calling process's pid namespace is nested strictly deeper than the pid namespace
// the tty's controlling session was created in (`ns->level > pid->level`) - a real, intentional,
// documented kernel behavior (not a hypothetical), reachable whenever a tty fd that has a
// controlling session crosses into a more deeply pid-namespaced process (e.g. a container/sandbox
// manager holding a tty open while a nested, more-deeply-namespaced child inherits or receives the
// fd and calls `tcgetsid` on it).
//
// This binary demonstrates two things with REAL kernel syscalls, end to end:
//   1. The normal, successful case: a real pty with a real attached controlling session really
//      does make `tcgetsid()` return a genuine nonzero sid via rustix's own public API - the
//      vulnerable function is live and exercised, not dead code.
//   2. An attempt to construct the actual cross-pid-namespace condition for real (so the repro
//      doesn't have to rely on kernel-source reasoning alone) - this sandbox blocks
//      `unshare(CLONE_NEWUSER)`/`unshare(CLONE_NEWPID)` with EPERM (verified directly below), so
//      step 2 cannot be completed here; `miri_ub` isolates the one remaining step (the
//      `Pid::from_raw_unchecked` construction itself) with the value kernel source proves is
//      reachable.
use rustix::fd::AsFd;
use std::ffi::CStr;
use std::os::unix::io::{AsRawFd, FromRawFd};

fn main() {
    // --- Step 1: genuine success case, through rustix's real public API ---
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
        eprintln!("[repro] opened pty master, slave = {name}");

        // Fork a child that detaches from its current session, starts a new one, and opens the
        // slave as its controlling terminal (TIOCSCTTY) - this is the real, standard way a
        // process acquires a controlling tty with a real, non-null `ctrl.session`.
        let pid = libc::fork();
        assert!(pid >= 0, "fork failed");
        if pid == 0 {
            // Child.
            if libc::setsid() < 0 {
                eprintln!("[child] setsid failed: {}", std::io::Error::last_os_error());
                std::process::exit(2);
            }
            let slave_fd = libc::open(name_ptr, libc::O_RDWR);
            if slave_fd < 0 {
                eprintln!("[child] open(slave) failed: {}", std::io::Error::last_os_error());
                std::process::exit(2);
            }
            if libc::ioctl(slave_fd, libc::TIOCSCTTY as _, 0) < 0 {
                eprintln!("[child] TIOCSCTTY failed: {}", std::io::Error::last_os_error());
                std::process::exit(2);
            }
            // Call the REAL public rustix API on the slave fd (now our controlling terminal,
            // with a real, non-null ctrl.session set to our own session == our own pid).
            let slave = std::fs::File::from_raw_fd(slave_fd);
            match rustix::termios::tcgetsid(slave.as_fd()) {
                Ok(sid) => {
                    eprintln!("[child] rustix::termios::tcgetsid() = {:?}", sid);
                    let own_pid = libc::getpid();
                    assert_eq!(
                        sid.as_raw_pid(),
                        own_pid,
                        "expected our own session id (we called setsid())"
                    );
                    std::process::exit(0);
                }
                Err(e) => {
                    eprintln!("[child] tcgetsid failed: {e}");
                    std::process::exit(2);
                }
            }
        } else {
            // Parent: wait for the child and check its exit status.
            let mut status: i32 = 0;
            libc::waitpid(pid, &mut status, 0);
            let ok = libc::WIFEXITED(status) && libc::WEXITSTATUS(status) == 0;
            eprintln!(
                "[parent] child exit status = {status:#x}, WIFEXITED={}, WEXITSTATUS={}",
                libc::WIFEXITED(status),
                libc::WEXITSTATUS(status)
            );
            assert!(ok, "child did not successfully call tcgetsid()");
            eprintln!(
                "[repro] Step 1 PASSED: real tcgetsid() through rustix's public API returns a \
                 genuine nonzero sid for a real attached session - the function is live, not \
                 dead code."
            );
        }
    }

    // --- Step 2: attempt the real cross-pid-namespace condition ---
    eprintln!("\n[repro] Step 2: attempting to construct a real cross-pid-namespace scenario...");
    let rc = unsafe { libc::unshare(libc::CLONE_NEWUSER) };
    if rc == 0 {
        eprintln!(
            "[repro] unshare(CLONE_NEWUSER) unexpectedly SUCCEEDED in this environment - a \
             fuller repro constructing the real pid_nr_ns()==0 condition would be possible here \
             and should be built as a follow-up."
        );
    } else {
        let errno = std::io::Error::last_os_error();
        eprintln!(
            "[repro] unshare(CLONE_NEWUSER) failed as expected in this sandbox: {errno} - \
             cannot construct the real kernel cross-pid-namespace condition that makes \
             tiocgsid's pid_vnr() return 0 (see RESULT.md). Falling back to miri_ub, which \
             isolates the one remaining step (Pid::from_raw_unchecked) with the exact value \
             kernel source (drivers/tty/tty_jobctrl.c::tiocgsid, kernel/pid.c::pid_nr_ns) \
             proves is reachable."
        );
    }
}
