// Same real pty / real kernel-produced pid=0 as before.rs, but exercising the PATCHED logic
// (patch.diff: the `if pid == 0 { return Err(OPNOTSUPP) }` check generalized to every platform,
// not just `#[cfg(linux_kernel)]`) - mirrors the patched function body verbatim, minus the cfg
// gate it removes.
use rustix::io::Errno;
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

        let pgrp = libc::tcgetpgrp(master.as_raw_fd());
        eprintln!("[repro] real libc::tcgetpgrp(master_fd) returned {pgrp}");
        pgrp
    }
}

fn patched_tcgetpgrp_body(pid: i32) -> Result<Pid, Errno> {
    // Patched: the pid==0 check now runs on every platform, not just `#[cfg(linux_kernel)]`.
    if pid == 0 {
        return Err(Errno::OPNOTSUPP);
    }
    Ok(unsafe { Pid::from_raw_unchecked(pid) })
}

fn main() {
    let pid = real_kernel_tcgetpgrp_zero();
    assert_eq!(
        pid, 0,
        "expected the real kernel to return 0 on an unattached pty master - got {pid} instead; \
         rerun, this is environment-dependent"
    );

    match patched_tcgetpgrp_body(pid) {
        Ok(p) => {
            // Would mean the fix failed to catch pid==0 - should never happen.
            println!("BUG STILL PRESENT: constructed {:?} from pid==0", p);
            std::process::exit(1);
        }
        Err(e) => {
            println!(
                "did not reproduce - patched code correctly rejected pid==0 with {:?}, no \
                 `Pid::from_raw_unchecked(0)` was ever constructed",
                e
            );
        }
    }
}
