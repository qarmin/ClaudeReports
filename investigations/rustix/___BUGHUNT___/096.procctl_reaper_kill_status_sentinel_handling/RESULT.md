# procctl_reaper_kill_status_sentinel_handling

`src/process/procctl.rs`'s FreeBSD `procctl(PROC_REAP_*)` bindings: the candidate suspected two
things, by analogy with this project's already-confirmed "NonZero*-backed type constructed from
raw/unvalidated kernel data without going through a fallible/checked path" bug shape (candidates
019/035/063/064/065/078/080) - (1) `get_reaper_status`'s `rs_reaper` field having no sentinel check
unlike the adjacent `rs_pid` (which explicitly checks FreeBSD's `-1` "no such descendant"
sentinel), and (2) `reaper_kill`'s `first_failed: Pid::from_raw(req.rk_fpid)` "silently mapping 0
to None, undocumented" as the mirror-image case.

## What was tried

| Approach/tool | Config (build/input/duration) | Outcome |
|---|---|---|
| Real FreeBSD kernel source read (`reap_status()`, `get_reaper_or_p()`, `sys/kern/kern_procctl.c`, fetched live via `curl` from `cgit.freebsd.org`) | n/a | `rs->rs_reaper = reap->p_pid`, where `reap` is always either `p` itself or `p->p_reaper` (every live process has one, defaulting to/terminating at `initproc`, pid 1) - never NULL, never pid 0. **No code path sets `rs_reaper` to 0 or -1.** The `rs_reaper` half of the candidate's premise does not hold - see "Root cause" below |
| Real FreeBSD kernel source read (`reap_kill()`, `reap_kill_proc_locked()`, `reap_kill_children()`, same file) | n/a | `reap_kill()` sets `rk->rk_fpid = -1` **unconditionally, before any signalling is attempted**, and only overwrites it with a real positive pid inside an `if (error != 0)` branch (an actual signal-delivery failure). So **`-1`, not `0`, is the kernel's real "nothing failed" sentinel** for `rk_fpid` - the candidate's "0 silently mapped to None" guess for this field was also not quite right, but reading the real kernel source surfaced the real, confirmed bug: no `-1` check at all |
| `git log -S`/`git show` on `src/process/procctl.rs` and `src/pid.rs` | `git log -S"rk_fpid"`, `git show a5e1f482`, `git show eb68ba30` | Confirmed a real regression: `reaper_kill`'s original introduction (a5e1f482, "Add more procctls for FreeBSD (#581)") *did* have the correct check (`if req.rk_fpid == -1 { None } else { unsafe { Pid::from_raw(req.rk_fpid) } }`) - eb68ba30 ("Reconsider guarantees of Pid and refactor (#657)") deleted that check and replaced it with the bare `Pid::from_raw(req.rk_fpid)` present at current HEAD, in the same commit that also gave `Pid::from_raw` a `debug_assert!(raw >= 0)` it didn't have before |
| Manual boundary-value construction: drive the real, portable `rustix::process::Pid::from_raw` (the exact function the vulnerable line calls) with the literal kernel-proven sentinel value (`-1`), in both debug and release builds, 3 repeat runs each | host x86_64 Linux, `repro/` (depends on the real crate via a path dependency) | **3/3 debug runs**: real panic, `src/pid.rs:44:9: assertion failed: raw >= 0`, identical location every time. **3/3 release runs**: identical wrong output every time, `first_failed = Some(Pid(-1))` for a fully successful (0-failure) kill |
| Cross-compile check (`cargo check --target x86_64-unknown-freebsd --features "std,all-apis"`) against both the unpatched and patched tree | host x86_64 Linux, `rustc 1.98.1`, freebsd target pre-installed | Both compile cleanly - confirms the vulnerable/fixed `reaper_kill` source is real, live code for the actual affected platform, not dead/unreachable code, and that the patch doesn't break compilation for it |
| Project test suite, before vs. after (same commit, `src/process/procctl.rs` toggled via `git stash`/`git stash pop` in the isolated worktree) | `cargo test --workspace --features "std,all-apis,use-libc" --no-fail-fast`, host x86_64 | **411 passed / 1 failed, identical both before and after** - the one failure (`ioctl::test_int_setter`, `NotFound` opening `/dev/net/tun`) is the same pre-existing, environment-caused failure documented for every other candidate on this project; no new failures from the fix |

## Status

Mixed result across the two sub-suspects in the candidate description:

- **`get_reaper_status`'s `rs_reaper`**: **Not reproducible**. Real kernel source proves `rs_reaper`
  is always a real, positive pid; there is no path that sets it to 0 or -1, so
  `Pid::from_raw(raw.rs_reaper).ok_or(io::Errno::RANGE)?` is correct as written.
- **`reaper_kill`'s `first_failed`**: **Confirmed**. Reproduces deterministically, 3/3 runs each in
  both debug (panics) and release (silently wrong value) builds, driving the real public
  `Pid::from_raw` with the literal value the real FreeBSD kernel source proves it receives on
  every ordinary successful `reaper_kill` call.

**Fully reproducible**: **No** - CONFIRMATION.md section 11's hardware exception. The affected
function (`reaper_kill`) is `#[cfg(freebsdlike)]`-gated and this sandbox is Linux-only; there is no
FreeBSD hardware/VM available to invoke the real `PROC_REAP_KILL` syscall end-to-end (the same
constraint already documented for candidates 035/063/064/065/078/080 on this project - a FreeBSD
userland binary cannot simply be run under Linux/QEMU-user, since FreeBSD's syscall ABI differs
from Linux's, not just the instruction set). What *was* obtained for real instead: (a) the real
kernel source read directly (not guessed) proving the exact sentinel value and when it's produced,
(b) the exact vulnerable/fixed source confirmed to compile for the real target, and (c) the real,
portable `Pid::from_raw` - the exact function the vulnerable line calls - fed that exact value and
shown, deterministically (3/3 runs, both build profiles), to panic (debug) or silently misreport
success as a failure (release).

## Class

logic (release build) / crash (debug build, via `Pid::from_raw`'s own `debug_assert`)

## Root cause

FreeBSD's `reap_kill()` (`sys/kern/kern_procctl.c`) sets `rk->rk_fpid = -1` unconditionally before
attempting to signal anything, and only overwrites it with a real target pid if that specific
process couldn't be signalled - i.e. `-1` is the kernel's own "0 failures" sentinel for this field,
exactly analogous to the adjacent `rs_pid` field in `procctl_reaper_status` (`-1` = "no such
descendant"), which `get_reaper_status` two functions above already special-cases explicitly.
`reaper_kill`'s own construction of `first_failed` (current HEAD) has no such check:

    first_failed: Pid::from_raw(req.rk_fpid),

`Pid::from_raw` (`src/pid.rs`) is `pub const fn from_raw(raw: RawPid) -> Option<Self>` with a
`debug_assert!(raw >= 0)` and `NonZeroI32::new(raw)` beneath it - in a debug build, feeding it `-1`
panics outright; in a release build (where `debug_assert!` is compiled out), `NonZeroI32::new(-1)`
succeeds, silently producing `Some(Pid(-1))`. So on every ordinary, fully successful `reaper_kill`
call (the common case - no process failed to be signalled), the current code either panics (debug
builds) or reports a nonsensical "process -1 failed to be signalled" (release builds) instead of
the correct `first_failed: None`.

This is a genuine regression, not a long-standing gap: the function's original introduction
(a5e1f482, "Add more procctls for FreeBSD (#581)") had the correct explicit check from the start.

## Introduced by

eb68ba30 "Reconsider guarantees of Pid and refactor (#657)" - this commit removed the pre-existing
`if req.rk_fpid == -1 { None } else { unsafe { Pid::from_raw(req.rk_fpid) } }` check (present since
`reaper_kill`'s original introduction) and replaced it with the bare `Pid::from_raw(req.rk_fpid)`
still at current HEAD, in the same commit that also added `Pid::from_raw`'s `debug_assert!(raw >=
0)` (previously `from_raw` was `unsafe fn` with no such assertion). Verified directly via `git show
eb68ba30 -- src/process/procctl.rs`.

## The fix

Mirror `get_reaper_status`'s own `rs_pid == -1` handling for `reaper_kill`'s `rk_fpid`:

```diff
     unsafe { procctl(PROC_REAP_KILL, process, as_mut_ptr(&mut req).cast())? };
     Ok(KillResult {
         killed: req.rk_killed as _,
-        first_failed: Pid::from_raw(req.rk_fpid),
+        // The kernel uses `-1` as the "no process failed to be signalled"
+        // sentinel for `rk_fpid` (it sets `rk_fpid = -1` unconditionally
+        // before attempting any signal delivery, only overwriting it with a
+        // real pid on failure), not `0` - `Pid::from_raw` cannot accept a
+        // negative value, so it must be special-cased the same way
+        // `get_reaper_status`'s `rs_pid` is above.
+        first_failed: if req.rk_fpid == -1 {
+            None
+        } else {
+            Pid::from_raw(req.rk_fpid)
+        },
     })
 }
```

**Checked against more than the one triggering input (CONFIRMATION.md section 7)**: re-ran the
real repro's "fixed formula" path against the exact same kernel-proven sentinel value (-1) and
confirmed it now returns `None` (correct); the repro also exercises the complementary case
(a positive `rk_fpid`, i.e. an actual failure) implicitly via the `else` branch being identical to
the pre-existing, never-disputed behavior for that case (unchanged by this patch).

- Cross-compile (both unpatched and patched, real `x86_64-unknown-freebsd` target): clean, no new
  warnings/errors.
- Project test suite (`cargo test --workspace --features "std,all-apis,use-libc" --no-fail-fast`,
  host x86_64): **411 passed, 1 failed, identical before and after** (the one failure is the
  pre-existing `/dev/net/tun` environment gap documented for every other candidate on this
  project; no new failures introduced by the fix).

## What's not done / open questions

- Not independently confirmed against a real FreeBSD kernel/syscall (no FreeBSD hardware/VM in
  this environment - see "Fully reproducible" above for exactly what was obtained instead).
- `get_reaper_pids` (`PROC_REAP_GETPIDS`, same file) also constructs `Pid::from_raw(raw.pi_pid)`/
  `Pid::from_raw(raw.pi_subtree)` from kernel-filled data with no sentinel handling - not checked
  this session; the real kernel's `reap_getpids()` path wasn't read, so whether those two fields
  can legitimately be 0/-1 is an open question for a future candidate, not resolved here.
- Did not check whether `rk_subtree`'s own encoding (user-supplied, not kernel-reported) has any
  analogous concern - out of scope for this candidate, which is specifically about kernel-reported
  sentinel values.
