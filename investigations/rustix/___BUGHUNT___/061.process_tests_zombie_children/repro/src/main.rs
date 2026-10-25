// Standalone repro for candidate 061 (process_tests_zombie_children).
//
// clippy::zombie_processes flags `tests/process/wait.rs` at lines 14, 41, 69, 104: each test
// spawns a `std::process::Child` via `Command::spawn()` and never calls `.wait()`/`.try_wait()`
// on that `Child` handle. The question this repro answers empirically: does that actually leave
// a zombie (defunct) process behind at the OS level, given that every one of those tests instead
// reaps the same pid through `rustix::process::waitpid`/`waitid` (a *different* API operating on
// the same pid, which issues the real wait4/waitid syscall)?
//
// This exercises the real end-to-end path: a real `Command::spawn()`, real SIGSTOP/SIGKILL
// delivery, and the real `rustix::process::waitpid` call - the exact sequence
// `test_waitpid_none` (tests/process/wait.rs:11-36) performs - then inspects the real `/proc`
// state of that exact pid afterwards.
//
// Exit code convention: 0 = did not reproduce (no zombie left behind - rustix's own wait call
// fully reaps the child even though `std::process::Child` is never `.wait()`-ed), non-zero =
// reproduced (a zombie was left behind).

use std::process::{Command, Stdio};
use std::{thread, time::Duration};

fn proc_state(pid: i32) -> Option<char> {
    let stat = std::fs::read_to_string(format!("/proc/{pid}/stat")).ok()?;
    // Format: "pid (comm) state ...". `comm` can contain spaces/parens, so split after the last ')'.
    let after_comm = stat.rsplit_once(')')?.1;
    after_comm.trim_start().chars().next()
}

/// Mirrors `test_waitpid_none` in tests/process/wait.rs exactly: spawns a child, SIGSTOPs it,
/// reaps the stop via `rustix::process::waitpid`, SIGKILLs it, reaps the exit the same way - and
/// crucially never calls `.wait()` on the `std::process::Child` handle itself (the pattern
/// clippy::zombie_processes flags).
fn run_rustix_reaped_case() -> i32 {
    let child = Command::new("yes")
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .spawn()
        .expect("failed to execute child");
    let pid = child.id() as i32;

    unsafe { libc::kill(pid, libc::SIGSTOP) };

    let (reaped_pid, status) = rustix::process::waitpid(None, rustix::process::WaitOptions::UNTRACED)
        .expect("failed to wait")
        .unwrap();
    assert_eq!(reaped_pid, rustix::process::Pid::from_child(&child));
    assert!(status.stopped());

    unsafe { libc::kill(pid, libc::SIGKILL) };

    let (reaped_pid, status) = rustix::process::waitpid(None, rustix::process::WaitOptions::UNTRACED)
        .expect("failed to wait")
        .unwrap();
    assert_eq!(reaped_pid, rustix::process::Pid::from_child(&child));
    assert!(status.signaled());

    // `child` (the std::process::Child) is dropped right here at the end of this function,
    // WITHOUT ever calling `.wait()`/`.try_wait()` on it - exactly what clippy flags.
    pid
}

/// Control case: spawn a child, kill it, and deliberately never wait on it through ANY API -
/// neither `std::process::Child::wait()` nor `rustix::process::waitpid`. This is what an actual
/// zombie looks like at the OS level, for comparison.
fn run_never_reaped_case() -> i32 {
    let child = Command::new("yes")
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .spawn()
        .expect("failed to execute child");
    let pid = child.id() as i32;
    unsafe { libc::kill(pid, libc::SIGKILL) };
    // Deliberately drop `child` without waiting on it through any mechanism.
    drop(child);
    pid
}

fn main() {
    let rustix_reaped_pid = run_rustix_reaped_case();
    // Give the kernel a moment to finish updating process-table bookkeeping.
    thread::sleep(Duration::from_millis(200));
    let state_after_rustix_wait = proc_state(rustix_reaped_pid);
    println!(
        "case 1 (rustix::process::waitpid reaps the pid, Child::wait() never called): \
         /proc/{rustix_reaped_pid}/stat state = {state_after_rustix_wait:?} \
         (None means the process is fully gone from the process table)"
    );

    let never_reaped_pid = run_never_reaped_case();
    thread::sleep(Duration::from_millis(200));
    let state_after_no_wait = proc_state(never_reaped_pid);
    println!(
        "case 2 (control: no wait call of any kind): /proc/{never_reaped_pid}/stat state = {state_after_no_wait:?} \
         ('Z' means it is a real zombie/defunct process)"
    );
    // Clean up the control case's zombie so it doesn't linger past this process's own lifetime -
    // this process is its parent, so a plain waitpid(2) on it is enough.
    unsafe {
        let mut status = 0i32;
        libc::waitpid(never_reaped_pid, &mut status, 0);
    }

    let reproduced = state_after_rustix_wait == Some('Z');
    if reproduced {
        eprintln!(
            "REPRODUCED: rustix::process::waitpid left a real zombie behind even though it \
             reported a successful wait - tests/process/wait.rs's pattern is a genuine zombie risk."
        );
        std::process::exit(1);
    } else {
        println!(
            "did not reproduce: rustix::process::waitpid fully reaped the child at the OS level \
             (no /proc entry remains), confirming clippy::zombie_processes is a false positive \
             here - it only sees that std::process::Child::wait() was never called, not that the \
             same pid was already reaped through rustix's own wait call."
        );
    }
}
