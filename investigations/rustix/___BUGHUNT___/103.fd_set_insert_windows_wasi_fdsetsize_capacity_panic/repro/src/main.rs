use rustix::event::{fd_set_insert, fd_set_num_elements, FdSetElement};
use rustix::fd::RawFd;

// The real `FD_SETSIZE` the target's own `FD_SET`/`fd_array` is fixed at -
// 1024 for wasi (libc's `wasi::FD_SETSIZE`), 64 for Windows
// (`windows_sys`'s `WinSock::FD_SETSIZE`).
#[cfg(target_os = "wasi")]
const FD_SETSIZE: usize = 1024;
#[cfg(windows)]
const FD_SETSIZE: usize = 64;

// Exit codes (chosen to avoid any stdout/stderr writes, which Miri's wasi
// shim cannot execute - see output.txt for why).
const EXIT_SETUP_PANICKED: i32 = 10;
const EXIT_FIRST_64_PANICKED: i32 = 11;
const EXIT_65TH_PANICKED: i32 = 12; // the candidate bug
const EXIT_NO_BUG: i32 = 0;

fn main() {
    // Suppress the default panic hook's own stderr write - Miri's wasi shim
    // can't execute `write` at all (see output.txt), so even a *caught*
    // panic would otherwise abort the whole run before we get to report a
    // clean exit code.
    std::panic::set_hook(Box::new(|_| {}));

    let result = std::panic::catch_unwind(|| {
        // Size a buffer for exactly FD_SETSIZE fds, as a caller following
        // the documented `fd_set_num_elements(set_count, nfds)` convention
        // would do.
        let set_count = FD_SETSIZE;
        let nelems = fd_set_num_elements(set_count, 100_000);
        let mut fds = vec![FdSetElement::default(); nelems];

        // Insert FD_SETSIZE distinct fds (0..FD_SETSIZE) - fills the set
        // exactly to its documented/intended capacity.
        for fd in 0..FD_SETSIZE as RawFd {
            fd_set_insert(&mut fds, fd);
        }

        fds
    });

    let mut fds = match result {
        Ok(fds) => fds,
        Err(_) => std::process::exit(EXIT_FIRST_64_PANICKED),
    };

    // Now insert one more, not-yet-present fd. This is the panic candidate.
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        fd_set_insert(&mut fds, 90_000 as RawFd);
    }));

    match result {
        Ok(()) => std::process::exit(EXIT_NO_BUG),
        Err(_) => std::process::exit(EXIT_65TH_PANICKED),
    }
}
