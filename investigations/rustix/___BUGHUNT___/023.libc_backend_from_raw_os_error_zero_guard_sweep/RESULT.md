# libc_backend_from_raw_os_error_zero_guard_sweep

Follow-up sweep from candidate 002's own "What's not done" note: three call sites construct
`Errno::from_raw_os_error` from a raw C-library/Winsock value with no pre-check for `0` -
`src/backend/libc/pty/syscalls.rs:100`, `src/backend/libc/termios/syscalls.rs:528`, and
`src/net/wsa.rs:27`/`:46` - mirroring the shape of 002's confirmed `linux_raw`-backend panic, but
these three sites all resolve to the `libc` backend's `Errno::from_raw_os_error`, not `linux_raw`'s.

## What was tried

| Approach/tool | Config (build/input/duration) | Outcome |
|---|---|---|
| Direct code reading | `src/backend/libc/io/errno.rs` `from_raw_os_error`/`Errno` struct definition; both other backends (`src/backend/linux_raw`) confirmed to be the only two `Errno` definitions in the crate | `libc` backend's `from_raw_os_error(raw: i32) -> Self { Self(raw) }` is an unconditional tuple-struct wrap with no `assert`/`debug_assert`/branch of any kind - there is no code path inside it that can panic for any `i32` input, by construction |
| Direct code reading of all 3 call sites | `pty/syscalls.rs` `ptsname()` (both the `apple` `ptsname_r`/`ioctl` fallback and the `linux_like`/`freebsd`/`fuchsia`/`illumos` `ptsname_r` path), `termios/syscalls.rs` `ttyname()`, `net/wsa.rs` `wsa_startup`/`wsa_cleanup` | `ptsname()`: every platform variant converges on `if r == 0 { return Ok(...) } if r != ERANGE { return Err(from_raw_os_error(r)) }` - `r == 0` is unconditionally filtered before the `from_raw_os_error` call, so `0` structurally cannot reach it. `ttyname()`: `match ttyname_r(...) { 0 => Ok(...), err => Err(from_raw_os_error(err)) }` - same structural guarantee via the match arm. `wsa.rs`: `WSAGetLastError()` is called only after `WSAStartup`/`WSACleanup` already returned nonzero (failure), with no guard against `WSAGetLastError()` itself returning `0`; this path is Windows-only (`#[cfg(windows)]` in `src/net/mod.rs`) so it also always resolves to the `libc`-backend `Errno` (the `linux_raw` backend is Linux-only) |
| Standalone repro, debug build, `rustix_use_libc` cfg forced | `cargo run` (dev profile, `debug-assertions = true`, `overflow-checks = true`), `RUSTFLAGS="--cfg=rustix_use_libc"`, values `{0, -1, 1, 2, 4096, i32::MIN, i32::MAX}` directly against the public `rustix::io::Errno::from_raw_os_error` | No panic/abort for any value, including `0`; every value round-trips losslessly through `raw_os_error()` (full `i32` storage, unlike `linux_raw`'s 16-bit encoding noted in 002) |

Only 2 of the available 3 escalating approaches were used: the function under test is a one-line,
branch-free tuple-struct constructor, so there is no ambiguity left for a 3rd tool (miri/fuzzing)
to resolve - static reading of the function body already rules out any panic path for any input,
and the empirical run is corroborating evidence, not the primary source of confidence.

## Status

**Not reproducible.** No panic is possible on the `libc` backend for any `Errno::from_raw_os_error`
input (confirmed by reading the unconditional one-line implementation, and empirically verified for
boundary values `0`, `-1`, `1`, `2`, `4096`, `i32::MIN`, `i32::MAX` with no panic in a
debug-assertions build). Additionally, 2 of the 3 named call sites (`pty/syscalls.rs`,
`termios/syscalls.rs`) cannot actually pass `0` through to `from_raw_os_error` at all - their own
match/if structure already filters the "success" (`0`) case into the `Ok` branch before the
`from_raw_os_error` call is ever reached, independent of what `from_raw_os_error` itself does.

## Class

N/A (not reproducible).

## Root cause

N/A - there is no bug. The `libc` backend's `Errno` is a `#[repr(transparent)]` wrapper around a
plain `c::c_int` (`src/backend/libc/io/errno.rs:37`) with `from_raw_os_error` defined as
`pub const fn from_raw_os_error(raw: i32) -> Self { Self(raw) }` - no assertion, no validation, no
niche/range invariant of any kind backs this type on the `libc` backend (unlike `linux_raw`'s
`Errno(u16)`, which candidate 002 showed panics via an `assert!(encoded >= 0xf001)` on out-of-range
input, including `0`). Since the `libc` backend never validates its stored value, there is no range
to violate and thus no panic path to trigger, regardless of whether an individual call site
pre-checks for `0` before calling it.

## Introduced by

N/A (not reproducible - no bug exists to date).

## The fix

No fix needed; there is nothing to fix. No patch.diff is included.

- Project test suite: not run before/after since no change was made to the source tree (N/A - see
  `SKILL.md` step 7, which only requires a before/after test run when a fix is applied).

## What's not done / open questions

- This finding was explicitly flagged (in candidate 002's own "What's not done" section) as "worth
  a second look if a similar range guard is ever added" to the `libc` backend's `Errno`. If a future
  change adds a range/niche invariant to `src/backend/libc/io/errno.rs`'s `Errno` (e.g. to unify
  storage with `linux_raw` or to reintroduce a `rustc_layout_scalar_valid_range_*` attribute), these
  three call sites (`pty/syscalls.rs:100`, `termios/syscalls.rs:528`, `net/wsa.rs:27`/`:46`) should
  be re-checked at that time, since one of them (`wsa.rs`) has no structural guard against `0`
  reaching `from_raw_os_error` the way the other two do.
- The `wsa.rs` call sites could not be exercised end-to-end on this (Linux) development machine
  since they are `#[cfg(windows)]`-gated and call real Win32 Winsock APIs (`WSAStartup`,
  `WSACleanup`, `WSAGetLastError`) - this is a platform-reachability limitation, not a reason to
  doubt the conclusion, since the destination function (`libc`-backend `from_raw_os_error`) is
  platform-independent and was directly verified not to panic for `raw == 0` regardless of which
  platform's C library's error value happens to be `0`.
