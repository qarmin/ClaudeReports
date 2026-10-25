# nonzero_backed_type_kernel_data_sweep

Meta/systemic sweep for more instances of the already-3x-confirmed "NonZero*-backed type
constructed from raw/unvalidated kernel data without going through a fallible `::from_raw`
constructor" pattern (candidates 019, 063, 064, 065), beyond the `thread::Pid`/`UCred` sites those
checked. Grepped every `NonZero*`/`new_unchecked`/`from_raw_unchecked`/`from_raw_nonzero_unchecked`
site in the crate and traced each one's data source. Found a new instance:
`tcgetsid()` (both the `libc` and `linux_raw` backends, `src/backend/{libc,linux_raw}/termios/
syscalls.rs`) constructs `Pid::from_raw_unchecked(pid)` from the raw `TIOCGSID` ioctl result with
**no zero-check at all**, unlike its sibling `tcgetpgrp()` (candidate 035's confirmed bug, where
the `linux_raw` backend checks unconditionally and the `libc` backend checks only under
`#[cfg(linux_kernel)]`) - `tcgetsid` has zero guard in either backend, including the crate's own
default Linux backend, not just the non-Linux platforms 035 was scoped to.

## What was tried

| Approach/tool | Config (build/input/duration) | Outcome |
|---|---|---|
| Crate-wide grep for every `NonZero*` type and every `new_unchecked`/`from_raw_unchecked`/`from_raw_nonzero_unchecked` call site, cross-checked each site's data source (user input vs. raw kernel/syscall data) | `grep -rn "NonZero\|new_unchecked\|from_raw_unchecked\|from_raw_nonzero_unchecked" src/` | Most sites are either user-supplied values (fadvise `len`, futex `val3`), compile-time signal constants, or `getpid`/`gettid`-style syscalls the kernel can never return 0 for; `SocketAddrAny::new`'s `NonZeroU32::new_unchecked(len)` is protected by its own preceding `assert!(len >= sockaddr_header size)`; `RawProtocol`/`Protocol::from_raw` already goes through the fallible `NonZeroU32::new`. `tcgetsid` stood out as the one site with literally no check in either backend, as its own sibling `tcgetpgrp` does have (candidate 035) |
| Code read: compare `tcgetsid`'s body against `tcgetpgrp`'s in both backends | `src/backend/libc/termios/syscalls.rs:346-351`, `src/backend/linux_raw/termios/syscalls.rs:249-256` | Confirmed: both `tcgetsid` implementations do `ret_pid_t(...)?`/`assume_init()` straight into `Pid::from_raw_unchecked(pid)` with no `pid == 0` check at all - `tcgetpgrp` has one in both backends (035) |
| Real Linux kernel source read (`torvalds/linux` master, fetched live): `drivers/tty/tty_jobctrl.c::tiocgsid`/`tiocgpgrp`, `kernel/pid.c::pid_nr_ns`/`pid_vnr` | `curl` against `raw.githubusercontent.com/torvalds/linux/master/...` | Confirmed: `tiocgsid()` returns `-ENOTTY` if `real_tty->ctrl.session` is NULL, otherwise `sid = pid_vnr(real_tty->ctrl.session)`. `pid_nr_ns()` returns `0` whenever `ns->level > pid->level` - i.e. whenever the calling process' pid namespace is nested *deeper* than the pid namespace the tty's controlling session was created in. This is a real, intentional, documented kernel mechanism (not hypothetical) - reachable whenever a tty fd with an established controlling session crosses into a more deeply pid-namespaced process (e.g. a container/sandbox manager holding a tty open while a nested, more-deeply-namespaced process inherits or receives the fd and calls `tcgetsid` on it) |
| Real end-to-end syscall test (`repro/src/bin/real_tcgetsid.rs`): real `posix_openpt`/`grantpt`/`unlockpt` pty, forked child does a real `setsid()` + `open(slave)` + `TIOCSCTTY`, then calls the REAL public `rustix::termios::tcgetsid()` on it | host x86_64 Linux, debug build, 3 repeat runs | **3/3 runs**: real attached session -> real nonzero sid through rustix's own public API (confirms the function is live/exercised, not dead code, for the common case) |
| Attempt to construct the real cross-pid-namespace condition directly: `unshare(CLONE_NEWUSER)` / `unshare --user --pid --mount-proc --fork` | host x86_64 Linux, this sandbox | **Fails with `EPERM` every time** (verified both via the repro binary's own `libc::unshare` call and directly via the `unshare` CLI) - this sandbox blocks unprivileged user/pid namespace creation entirely (same restriction already documented for candidates 057/063/064/065/066-068), so the real end-to-end cross-namespace scenario cannot be constructed here |
| Miri, isolating the final construction step (`Pid::from_raw_unchecked(0)`, the exact value kernel source proves `tiocgsid`'s `pid_nr_ns()` returns under the cross-namespace condition) | `cargo +nightly miri run --release --bin miri_ub`, 3 repeat runs | **Real, deterministic Undefined Behavior, 3/3 runs**: `error: Undefined Behavior: entering unreachable code` at `NonZeroI32::new_unchecked(raw)` (`src/pid.rs:62`), identical each time |
| Project test suite, before vs. after the fix | `cargo test --workspace --features "std,all-apis,use-libc" --no-fail-fast`, host x86_64 | **411 passed / 1 failed, identical both before and after** - the one failure (`ioctl::test_int_setter`, `NotFound` opening `/dev/net/tun`) is the same pre-existing, environment-caused failure documented for every other candidate on this project; no new failures from the fix |

## Status

**Confirmed**. The vulnerable code path (`tcgetsid` in both backends constructing
`Pid::from_raw_unchecked(pid)` with no zero-check) is confirmed live/reachable via a real syscall
(3/3 runs), and the exact value kernel source proves is a legitimate, reachable return from the
real `TIOCGSID` ioctl under a real, documented kernel condition (`pid_nr_ns`'s cross-pid-namespace
`ns->level > pid->level` case) is confirmed, independently, to produce real Miri-detected
Undefined Behavior when fed into that exact construction (3/3 runs, deterministic).

**Fully reproducible**: **No** - CONFIRMATION.md section 11's hardware/privilege exception. This
sandbox blocks unprivileged `unshare(CLONE_NEWUSER)`/`unshare(CLONE_NEWPID)` with `EPERM`
(verified directly, not assumed - see "What was tried" above), so the real end-to-end
cross-pid-namespace scenario that makes the real kernel's `TIOCGSID` ioctl actually return `0`
cannot be constructed in this environment. This is weaker evidence than candidate 035's sibling
finding (which *did* get a genuine kernel-produced `0` from `tcgetpgrp` via a simpler mechanism -
an unattached pty, no privilege needed - and only lacked the ability to execute the resulting
macOS/BSD-compiled binary): here, neither the full cross-namespace scenario nor a genuine
kernel-produced `0` for `tcgetsid` specifically could be obtained at all in this sandbox. What
*was* obtained for real: (a) the vulnerable function is live and exercised via a real syscall for
the common case, and (b) the kernel source mechanism that makes it return 0 is read directly from
the actual kernel source (not inferred/guessed), cross-checked against the same `pid_nr_ns`
function whose cross-namespace zero-return behavior this project has already independently
confirmed is real and exploitable for a structurally identical bug shape (candidates 063/064/065,
`UCred.pid` via `SO_PEERCRED`/`SCM_CREDENTIALS`, which hit the exact same sandbox privilege wall
and used the same category of exception), and (c) the exact value that mechanism produces is
verified, mechanically, to cause real Miri-detected UB when constructed via the real
`Pid::from_raw_unchecked` call the vulnerable function performs.

## Class

UB

## Root cause

`Pid` (`src/pid.rs`) is `#[repr(transparent)] struct Pid(NonZeroI32)`, and
`Pid::from_raw_unchecked` is `unsafe fn` precisely because its body
(`NonZeroI32::new_unchecked(raw)`) skips the zero-check `Pid::from_raw` performs - the caller must
independently guarantee `raw != 0`. `tcgetsid`'s sibling `tcgetpgrp` (candidate 035) *does*
establish that guarantee before calling `from_raw_unchecked` (unconditionally in the `linux_raw`
backend; under `#[cfg(linux_kernel)]` in the `libc` backend), because the real kernel's
`tiocgpgrp()` can return a raw pid of `0` when the tty has no foreground process group at all
(`tty_get_pgrp()` returns `NULL`, and `pid_vnr(NULL)` is defined to be `0`). `tcgetsid` never
received the analogous guard in either backend, despite the real kernel's `tiocgsid()`
(`drivers/tty/tty_jobctrl.c`) going through the *same* `pid_vnr()`/`pid_nr_ns()` machinery
(`kernel/pid.c`) for the session id it reports - `pid_nr_ns()` returns `0` whenever the calling
process' pid namespace is nested strictly deeper than the namespace the tty's controlling session
was created in (`ns->level > pid->level`), a real, intentional kernel behavior (not a hypothetical
edge case), reachable whenever a controlling-terminal fd with an already-established session
crosses into a more deeply pid-namespaced process. When that happens, both `tcgetsid`
implementations construct `Pid::from_raw_unchecked(0)`, which is immediate Undefined Behavior (the
compiler is entitled to assume every `NonZeroI32`/`Pid` is never zero - the entire premise of the
niche optimization `Option<Pid>` relies on, per `src/pid.rs`'s own `assert_eq_size!` test), not
merely a logic error contingent on later use of the value.

## Introduced by

eb68ba30 "Reconsider guarantees of Pid and refactor (#657)" - this is the commit that made
`Pid::from_raw_unchecked`'s body call `NonZeroI32::new_unchecked` directly on an unvalidated raw
value (replacing an earlier `Pid::from_raw_nonzero(RawNonZeroPid::new_unchecked(pid))` form), which
is the specific mechanism this candidate confirms is real UB. The underlying omission - `tcgetsid`
never checking for a raw pid of 0, unlike `tcgetpgrp` - is actually older: `git show eb68ba30 --
src/backend/libc/termios/syscalls.rs` shows `tcgetsid` had only a non-enforcing
`debug_assert_ne!(pid, 0)` immediately before that commit (itself traceable back to the function's
original introduction in fb35a553 "Implement a new `rustix::termios` module. (#274)"), and
`tcgetpgrp`'s own zero-check was added later and scoped only to `tcgetpgrp` by bc4fa30d "Fix
`tcgetpgrp` and `tcsetpgrp` on Linux. (#910)" (cited by candidate 035) - `tcgetsid`'s analogous gap
was never addressed by that or any later commit.

## The fix

Add the same `if pid == 0 { return Err(io::Errno::OPNOTSUPP); }` guard `tcgetpgrp` already has, to
`tcgetsid` in both backends, before constructing the `Pid` - applied unconditionally (not gated by
`#[cfg(linux_kernel)]`) in the `libc` backend, matching candidate 035's own recommended fix for
`tcgetpgrp`'s cfg gap (the `Pid` invariant is unconditional, so the check establishing it should
be too; this crate has not independently verified the analogous mechanism exists on non-Linux
BSD/macOS kernels for `tcgetsid` specifically, but a defensive unconditional check costs nothing
on platforms where the kernel never returns 0 here).

```diff
 pub(crate) fn tcgetsid(fd: BorrowedFd<'_>) -> io::Result<Pid> {
     unsafe {
         let pid = ret_pid_t(c::tcgetsid(borrowed_fd(fd)))?;
+
+        // As with `tcgetpgrp` above, the kernel can report a session id of 0
+        // (e.g. on Linux, when the tty's controlling session isn't visible in
+        // the calling process' pid namespace), which `Pid::from_raw_unchecked`
+        // cannot accept.
+        if pid == 0 {
+            return Err(io::Errno::OPNOTSUPP);
+        }
+
         Ok(Pid::from_raw_unchecked(pid))
     }
 }
```
(and the structurally identical change in the `linux_raw` backend's `MaybeUninit`-based version).

**Checked against more than the one triggering input (CONFIRMATION.md section 7)**: re-ran the
real pty+session repro 3 times against the patched tree - the normal/common case (a real,
non-zero, correctly-attached session) still returns the correct sid unchanged, confirming the fix
doesn't disturb the success path it isn't meant to touch.

- Build-verified (patched tree, both backends): `cargo build --features "std,all-apis"` and
  `cargo build --features "std,all-apis,use-libc"` both succeed cleanly.
- Project test suite (`cargo test --workspace --features "std,all-apis,use-libc" --no-fail-fast`,
  host x86_64): **411 passed, 1 failed, identical before and after** (the one failure is the
  pre-existing `/dev/net/tun` environment gap documented for every other candidate on this
  project; no new failures introduced by the fix).

## What's not done / open questions

- Not independently confirmed on real kernel hardware with actual pid-namespace nesting (this
  sandbox blocks `unshare(CLONE_NEWUSER)`/`unshare(CLONE_NEWPID)` outright - see "Fully
  reproducible" above for exactly what was and wasn't verified instead, and why).
- The sweep that found this (grep across every `NonZero*`/`_unchecked` construction site) did not
  turn up any other new instance beyond this one and the already-confirmed 019/035/063/064/065 -
  but it was a single-session grep-and-trace pass, not an exhaustive type-level audit (e.g. it
  didn't attempt to trace every `MaybeUninit`-based struct field that might itself wrap a
  `NonZero*` type nested inside a larger kernel-filled struct, beyond the sites a plain-text grep
  for `NonZero`/`_unchecked` surfaces directly).
- Did not check whether the non-Linux `libc`-backend targets (macOS/BSD) have an analogous
  `tcgetsid`-returns-0 mechanism - the fix applied there is defensive/unconditional rather than
  based on a specific confirmed non-Linux kernel behavior for this specific ioctl (unlike
  `tcgetpgrp`, where issue #1678 already established the BSD/macOS angle for candidate 035).
