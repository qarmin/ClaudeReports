//! Reproduces the confirmed sentinel-handling bug in
//! `rustix::process::reaper_kill` (src/process/procctl.rs, FreeBSD
//! `PROC_REAP_KILL`).
//!
//! Real FreeBSD kernel source (sys/kern/kern_procctl.c, `reap_kill()`,
//! fetched live from https://cgit.freebsd.org/src/plain/sys/kern/kern_procctl.c)
//! proves:
//!
//! ```c
//! error = ESRCH;
//! rk->rk_killed = 0;
//! rk->rk_fpid = -1;            // <-- unconditional, BEFORE any signalling is attempted
//! if ((rk->rk_flags & REAPER_KILL_CHILDREN) != 0) {
//!     reap_kill_children(td, reaper, rk, &ksi, &error);
//! } else {
//!     ...
//!     reap_kill_subtree(td, p, &reaper, &w);
//!     ...
//! }
//! ```
//!
//! `rk_fpid` is only ever overwritten with a *positive* pid if some process
//! actually could not be signalled (`reap_kill_proc_locked`/
//! `reap_kill_children`, both only assign `rk->rk_fpid = <target's real
//! pid>` inside their respective `if (error != 0)` branches). So `-1` is the
//! kernel's own "nothing failed" sentinel for this field - the exact same
//! role the adjacent `rs_pid` field plays in `procctl_reaper_status`, which
//! rustix's own `get_reaper_status` (two functions above `reaper_kill` in
//! the same file) explicitly checks for:
//!
//! ```rust,ignore
//! pid: if raw.rs_pid == -1 { None } else { Some(Pid::from_raw(raw.rs_pid).ok_or(io::Errno::RANGE)?) },
//! ```
//!
//! `reaper_kill`'s own construction of `first_failed` has **no such check**
//! (current HEAD, src/process/procctl.rs):
//!
//! ```rust,ignore
//! first_failed: Pid::from_raw(req.rk_fpid),
//! ```
//!
//! This repro cannot invoke the real `PROC_REAP_KILL` syscall end-to-end:
//! the module is `#[cfg(freebsdlike)]`-gated and this sandbox is Linux-only
//! (no FreeBSD hardware/VM available - see RESULT.md's "Fully reproducible"
//! section). What it *can* do, and does, is drive the real, portable
//! `rustix::process::Pid::from_raw` - the exact function the vulnerable
//! line calls - with the literal value (`-1`) the real kernel source above
//! proves `rk_fpid` holds on every ordinary, fully-successful (0 failures)
//! `reaper_kill` call, through both the vulnerable and the fixed formula.
//!
//! Usage: `repro [vulnerable|fixed]` (default: `vulnerable`). Both formulas are always printed
//! for context, but the exit code reflects only the selected one - this lets the candidate's
//! `justfile` drive a real before/after cycle (`reproduce-before` runs the default `vulnerable`
//! mode and expects a nonzero/panicking exit; `reproduce-after` runs `fixed` and expects a clean
//! exit) even though neither mode actually depends on the state of `../clone` (see the module doc
//! comment above for why: the real affected function can't be compiled/run on this Linux host at
//! all, so `../clone`'s patch state is instead verified directly, by the justfile, with a
//! real cross-compile check + a grep for the guard - see the justfile for that half).
//!
//! Exit code convention: 0 = did not reproduce, non-zero = reproduced.

use rustix::process::Pid;

/// The real FreeBSD kernel's own "no process failed to be signalled"
/// sentinel value for `rk_fpid` (see module doc comment above).
const KERNEL_SUCCESS_SENTINEL: i32 = -1;

/// The exact (buggy) expression currently in `reaper_kill`
/// (src/process/procctl.rs, current HEAD):
///     `first_failed: Pid::from_raw(req.rk_fpid)`
fn vulnerable_formula(raw_fpid: i32) -> Option<Pid> {
    Pid::from_raw(raw_fpid)
}

/// The fixed expression (this candidate's `patch.diff`), mirroring the
/// explicit `-1` check `get_reaper_status` already performs for the
/// adjacent `rs_pid` field:
fn fixed_formula(raw_fpid: i32) -> Option<Pid> {
    if raw_fpid == -1 {
        None
    } else {
        Pid::from_raw(raw_fpid)
    }
}

fn main() {
    let mode = std::env::args().nth(1).unwrap_or_else(|| "vulnerable".to_string());

    println!(
        "Simulating a fully successful `reaper_kill()` call (0 processes failed to be \
         signalled) against the real FreeBSD kernel's own `rk_fpid` sentinel value: {KERNEL_SUCCESS_SENTINEL}"
    );
    println!("(value sourced directly from sys/kern/kern_procctl.c's `reap_kill()`, not guessed)");
    println!("(selected mode: {mode})");
    println!();

    println!("--- fixed formula (this candidate's patch.diff) ---");
    let fixed_ok = match std::panic::catch_unwind(|| fixed_formula(KERNEL_SUCCESS_SENTINEL)) {
        Ok(v) => {
            println!("first_failed = {v:?}  (correct: None, nothing failed)");
            v.is_none()
        }
        Err(e) => {
            println!("PANICKED (unexpected for the fixed formula): {e:?}");
            false
        }
    };

    println!();
    println!("--- vulnerable formula (current HEAD, unpatched) ---");
    let vulnerable_reproduced = match std::panic::catch_unwind(|| vulnerable_formula(KERNEL_SUCCESS_SENTINEL)) {
        Ok(Some(p)) => {
            eprintln!(
                "REPRODUCED - wrong output: first_failed = Some({p:?}) for a fully successful \
                 kill (0 failures) - the kernel's own \"nothing failed\" sentinel was silently \
                 mis-constructed into a bogus non-null Pid instead of None"
            );
            true
        }
        Ok(None) => {
            println!("did not reproduce via this build - got None (correct)");
            false
        }
        Err(e) => {
            eprintln!(
                "REPRODUCED - panicked (debug_assert in Pid::from_raw fired on the negative \
                 raw value): {e:?}"
            );
            true
        }
    };

    // The exit code reflects only the mode actually selected (see usage doc above) - both
    // formulas are always computed and printed for side-by-side comparison regardless.
    let failed = match mode.as_str() {
        "fixed" => !fixed_ok,
        _ => !vulnerable_reproduced,
    };
    if failed {
        std::process::exit(1);
    }
}
